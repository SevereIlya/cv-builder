use rfd::FileDialog;
use std::env;
use std::io;
use std::io::{Error, ErrorKind, Write};
use std::process::{Command, exit, Output};
use tracing::{error, info, warn, debug};

fn check_dependency(name: &str, install_hint: &str) -> anyhow::Result<()> {
    info!(
        dependency = name,
        "Проверка зависимостей"
    );

    match Command::new(name).arg("--version").output() {
        Ok(_) => {
            info!(
                dependency = name,
                "Зависимость найдена"
            );
            Ok(())
        }
        Err(e) => {
            error!(
                dependency = name,
                ?e,
                "Зависимость не найдена"
            );

            warn!(
                dependency = name,
                hint = install_hint,
                "Совет по установке"
            );

            anyhow::bail!("Missing dependency: {}", name)
        }
    }
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // ======================================================================================

    info!("Проверяем, зависимости...");

    let pandoc_hint = "Скачай с официального сайта: https://pandoc.org/installing.html";
    let latex_hint = "Установи MiKTeX (https://miktex.org/download). При установке выбери 'Install missing packages on-the-fly = Yes'.";

    if check_dependency("pandoc", pandoc_hint).is_err() ||
        check_dependency("lualatex", latex_hint).is_err() {
        error!("Не хватает инструментов. Пожалуйста, установи их и перезапусти программу.");
        pause()?;
        exit(1);
    }

    info!("Все зависимости установлены.");

    // ======================================================================================

    info!("Стартуем генератор CV...");

    let input_path = match FileDialog::new()
        .add_filter("Markdown", &["md"])
        .set_title("1/3: Выбери .md файл")
        .pick_file()
    {
        Some(path) => path,
        None => {
            warn!("Файл не выбран.");
            pause()?;
            return Ok(());
        }
    };

    let template_path = match FileDialog::new()
        .add_filter("LaTeX Template", &["tex"])
        .set_title("2/3: Выбери .tex файл")
        .pick_file()
    {
        Some(path) => path,
        None => {
            warn!("Шаблон не выбран.");
            pause()?;
            return Ok(());
        }
    };

    let output_path = match FileDialog::new()
        .add_filter("PDF", &["pdf"])
        .set_title("3/3: Куда положить готовый PDF?")
        .set_file_name("CV.pdf")
        .save_file()
    {
        Some(path) => path,
        None => {
            warn!("Место сохранения не выбрано.");
            pause()?;
            return Ok(());
        }
    };

    // ======================================================================================

    let parent_dir = input_path.parent().ok_or_else(|| {
        error!(
            input = ?input_path,
            "Не удалось определить родительский каталог"
        );
        Error::new(ErrorKind::Other, "Нет родительского каталога")
    })?;

    env::set_current_dir(parent_dir).map_err(|e| {
        error!(
            ?e,
            dir = ?parent_dir,
            "Не удалось изменить рабочий каталог"
        );
        e
    })?;

    // ======================================================================================

    let template_arg = format!("--template={}", template_path.display());

    info!(
        input = %input_path.display(),
        output = %output_path.display(),
        "Собираем PDF"
    );

    let output: Output = Command::new("pandoc")
        .args([
            input_path.to_str().expect("Кривой путь у входного файла"),
            &template_arg,
            "--pdf-engine=lualatex",
            "-o",
            output_path.to_str().expect("Кривой путь у выходного файла"),
        ])
        .output()
        .map_err(|e| {
            error!(?e, "Не удалось запустить pandoc");
            e
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("--- ПОДРОБНОСТИ ОШИБКИ (Pandoc/LaTeX) ---");
        eprintln!("{}", stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        error!(
            code = ?output.status.code(),
            stderr_len = stderr.len(),
            stdout_len = stdout.len(),
            "Pandoc failed"
        );

        debug!(%stdout, "Pandoc stdout");
        debug!(%stderr, "Pandoc stderr");
    }

    if !output_path.exists() {
        error!(
            file = ?output_path,
            "PDF-файл не был создан"
        );
        pause()?;
        exit(1);
    }

    info!("SUCCESS: Файл успешно создан!");
    pause()?;
    Ok(())
}

fn pause() -> anyhow::Result<()> {
    print!("Press Enter to exit...");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
