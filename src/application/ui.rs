use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table};
use tui::Frame;
use crate::application::state::State;
use crate::application::App;

pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)].as_ref())
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(chunks[1]);
    let rss_names = draw_list_rss(app.state());
    rect.render_widget(rss_names, body_chunks[0]);

    let body = draw_list_news(app.state());
    rect.render_widget(body, body_chunks[1]);

    let footer = draw_footer();
    rect.render_widget(footer, chunks[2]);
}

fn check_size(rect: &Rect) {
    if rect.width < 105 {
        panic!("Need width >= 105, got {}", rect.width);
    }
    if rect.height < 30 {
        panic!("Need height >= 30, got {}", rect.height);
    }
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("TUI - Lecteur RSS")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}

fn draw_list_news<'a>(state: &State) -> Table {
    let news_style = Style::default().fg(Color::White);
    let news_selected_style = Style::default().fg(Color::LightCyan);
    let mut rows = vec![];
    let mut i : u8 = 0;
    let _list_news = if let Some(list_news) = state.list_news() {
        for news in list_news.items.iter() {
            if Some(i)==state.news_selected() {
                let row = Row::new(vec![
                    Cell::from(Span::styled(news.title().unwrap(), news_selected_style))
                ]);
                rows.push(row);
            } else {
                let row = Row::new(vec![
                    Cell::from(Span::styled(news.title().unwrap(), news_style))
                ]);
                rows.push(row);
            }
            i=i+1;
        }
    };
    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Actus"),
        )
        .widths(&[Constraint::Length(100), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_list_rss(state: &State) -> Table {
    let rss_selected_style = Style::default().fg(Color::LightCyan);
    let rss_style = Style::default().fg(Color::White);
    let mut rows = vec![];
    let mut i : u8 = 0;
    let _list_rss = if let Some(list_rss) = state.list_rss() {
        for rss in list_rss.iter() {
            if Some(i) == state.rss_selected() {
                let row = Row::new(vec![
                    Cell::from(Span::styled(rss.name.to_string(), rss_selected_style))
                ]);
                rows.push(row);
            }else{
                let row = Row::new(vec![
                    Cell::from(Span::styled(rss.name.to_string(), rss_style))
                ]);
                rows.push(row);
            }
            i=i+1;
        }
    };
    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .title("Liste RSS"),
        )
        .widths(&[Constraint::Length(20), Constraint::Min(20)])
        .column_spacing(1)
}

fn draw_footer<'a>() -> Paragraph<'a> {
    Paragraph::new("Haut/Bas : Déplacement Flux RSS, Gauche/Droite : Déplacement News, Entrée : Voir la news, q : Quitter")
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Infos")
            .border_type(BorderType::Plain),
    )
}