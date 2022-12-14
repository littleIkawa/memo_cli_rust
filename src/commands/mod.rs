//! サブコマンドごとに実行関数を切り分けたモジュール.

use crate::{
    parser::{AppArgs, Subcommands},
    server::http_server,
};

use self::{
    copy::copy_command, edit::edit_command, grep::grep_command, info::info_command,
    list::list_command, new::new_command, remove::remove_command, view::view_command,
};

mod copy;
mod edit;
mod grep;
mod info;
mod list;
mod new;
mod remove;
mod view;

/// パースした構造体を受け取って各関数の処理を投げる関数.
pub(crate) fn execute_commands(args: &AppArgs) -> Result<(), Box<dyn std::error::Error>> {
    match &args.subcommands {
        Subcommands::New { name } => {
            let memo_name = new_command(name)?;
            // 作成後すぐ編集する
            edit_command(&Some(memo_name))?;
        }
        Subcommands::List { full } => {
            list_command(full)?;
        }
        Subcommands::Edit { name } => {
            edit_command(name)?;
        }
        Subcommands::View { name } => {
            view_command(name)?;
        }
        Subcommands::Remove { name } => {
            remove_command(name)?;
        }
        Subcommands::Grep { args } => {
            grep_command(args)?;
        }
        Subcommands::Copy { name, md, rename } => {
            copy_command(name, md, rename)?;
        }
        Subcommands::Serve => {
            http_server()?;
        }
        Subcommands::Info {
            version,
            storage,
            port,
        } => {
            info_command(version, storage, port);
        }
    }

    Ok(())
}
