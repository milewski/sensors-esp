use embedded_sdmmc::DirEntry;
use numfmt::{Formatter, Precision, Scales};
use profont::PROFONT_9_POINT;
use shared::tiny_display::TinyDisplay;

pub struct FileList<'a> {
    display: TinyDisplay<'a>,
    files: Vec<DirEntry>,
    offset: usize,
    limit: usize,
    formatter: Formatter,
}

impl<'a> FileList<'a> {
    pub fn new(display: TinyDisplay<'a>, files: Vec<DirEntry>) -> anyhow::Result<FileList<'a>> {
        let mut formatter = Formatter::new()
            .scales(Scales::new(1024, vec!["b", "k", "M", "G", "T", "P"])?)
            .precision(Precision::Significance(0));

        Ok(
            Self {
                display,
                files,
                formatter,
                offset: 0,
                limit: 6,
            }
        )
    }

    pub fn scroll_up(&mut self) -> anyhow::Result<()> {
        if self.can_scroll_up() {
            self.offset += 1;
            self.draw()?;
        }

        Ok(())
    }

    pub fn scroll_down(&mut self) -> anyhow::Result<()> {
        if self.can_scroll_down() {
            self.offset -= 1;
            self.draw()?;
        }

        Ok(())
    }

    pub fn can_scroll_up(&self) -> bool {
        self.files.len() - self.offset > self.limit
    }

    pub fn can_scroll_down(&self) -> bool {
        self.offset > 0
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.display.clear();

        let offset_top = 8;

        for (index, file) in self.files[self.offset..].iter().take(self.limit).enumerate() {
            let size = format!("{:>4}", self.formatter.fmt2(file.size));

            self.display.draw_text(&size, PROFONT_9_POINT, 0, offset_top + (10 * index as i32))?;
            self.display.draw_text(&file.name.to_string(), PROFONT_9_POINT, 30, offset_top + (10 * index as i32))?;
        }

        self.display.flush()
    }
}