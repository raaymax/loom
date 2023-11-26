use serde_json::Value;
use tower_lsp::jsonrpc;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use lexer::{Tokenizer, TokenVec, PError, Token};

#[derive(Debug)]
struct Backend {
    client: Client,
}
fn validate(path: &str) -> Vec<Diagnostic> {
    let mut v = Vec::new();
    //v.push(Diagnostic::new(Range::new(Position::new(2, 0),Position::new(2, 5)), Some(DiagnosticSeverity::ERROR), None, None, "error".to_string(), None, None));
    let text = {
        std::fs::read_to_string(path.clone()).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })
    };
    let tokensResult = Tokenizer::new(&text).collect::<Result<Vec<Token>, PError>>();
    if let Err(e) = tokensResult {
        let start = e.get_start();
        let end = e.get_end();
        v.push(Diagnostic::new(Range::new(Position::new(start.0 as u32 - 1, start.1 as u32),Position::new(end.0 as u32 - 1, end.1 as u32)), Some(DiagnosticSeverity::ERROR), None, None, e.message, None, None));
        return v;
    };

    let Ok(tokens) = tokensResult else {
        if let Err(e) = tokensResult {
            let start = e.get_start();
            let end = e.get_end();
            v.push(
                Diagnostic::new(
                    Range::new(
                        Position::new(start.0 as u32 - 1, start.1 as u32),
                        Position::new(end.0 as u32 - 1, end.1 as u32)
                    ),
                    Some(DiagnosticSeverity::ERROR), None, None, e.message, None, None
                )
            );
        };
        return v;
    };
    let mut iter = tokens.iter();
    if let Err(e) = parser::parse(&mut iter) {
        let start = e.get_start();
        let end = e.get_end();
        v.push(
            Diagnostic::new(
                Range::new(
                    Position::new(start.0 as u32 - 1, start.1 as u32),
                    Position::new(end.0 as u32 - 1, end.1 as u32)
                ), 
                Some(DiagnosticSeverity::ERROR), None, None, e.message, None, None
            )
        );
    }
    v
}
#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),

                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        ..Default::default()
                    },
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
    }

    async fn execute_command(&self, _: ExecuteCommandParams) -> jsonrpc::Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, _: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
    }

    async fn did_change(&self, _: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let v = validate(&params.text_document.uri.path().to_string());
        self.client.publish_diagnostics(params.text_document.uri.clone(), v, None).await;
        self.client.log_message(MessageType::ERROR, format!("file saved! {:?} {:?}", params.text_document.uri, params.text)).await;
    }

    async fn did_close(&self, _: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;
    }

    async fn completion(&self, _: CompletionParams) -> jsonrpc::Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "runtime-agnostic")]
    use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    #[cfg(feature = "runtime-agnostic")]
    let (stdin, stdout) = (stdin.compat(), stdout.compat_write());

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
