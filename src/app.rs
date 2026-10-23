use crate::components::header::header;
use crate::components::task_row::task_row;
use crate::components::toast::toast;
use crate::crypto::{get_or_create_key, init_keyring};
use crate::storage::{TASKS_FILE, backup_broken, load_tasks, save_tasks, tasks_path};
use crate::theme;
use crate::theme::tokens::{colors, spacing};
use crate::todo::TodoItem;
use chacha20poly1305::Key;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{column, container, scrollable, stack, text, text_input};
use iced::{Element, Length, Task};
use std::path::PathBuf;
use std::time::Duration;

const ERROR_DURATION: Duration = Duration::from_secs(3);

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    AddTask,
    ToggleTask(usize),
    DeleteTask(usize),
    HideError(u64),
}

#[derive(Default)]
pub struct TodoApp {
    input: String,
    tasks: Vec<TodoItem>,
    next_id: usize,
    tasks_path: PathBuf,
    key: Option<Key>,
    error: Option<String>,
    error_id: u64,
}

impl TodoApp {
    pub fn new() -> (TodoApp, Task<Message>) {
        let mut startup_error: Option<&str> = None;

        let tasks_path = tasks_path().unwrap_or_else(|e| {
            eprintln!("{:?}", e);
            PathBuf::from(TASKS_FILE)
        });
        let key = match init_keyring().and_then(|_| get_or_create_key()) {
            Ok(key) => Some(key),
            Err(e) => {
                eprintln!("{:#}", e);
                startup_error = Some("Не удалось получить ключ шифрования");
                None
            }
        };

        let tasks = match &key {
            Some(key) => match load_tasks(key, &tasks_path) {
                Ok(tasks) => tasks,
                Err(e) => {
                    eprintln!("{:#}", e);
                    match backup_broken(&tasks_path) {
                        Ok(backup) => eprintln!("Broken tasks file saved to {}", backup.display()),
                        Err(e) => eprintln!("{:#}", e),
                    }
                    startup_error = Some("Файл задач повреждён, сохранена резервная копия");
                    Vec::new()
                }
            },
            None => Vec::new(),
        };

        let next_id = tasks.iter().map(|t| t.id).max().map_or(0, |id| id + 1);

        let mut app = TodoApp {
            tasks,
            next_id,
            tasks_path,
            key,
            ..Default::default()
        };

        let task = match startup_error {
            Some(message) => app.show_error(message),
            None => Task::none(),
        };

        (app, task)
    }

    fn show_error(&mut self, text: impl Into<String>) -> Task<Message> {
        self.error_id += 1;
        self.error = Some(text.into());

        let id = self.error_id;
        Task::perform(tokio::time::sleep(ERROR_DURATION), move |_| {
            Message::HideError(id)
        })
    }

    fn save(&mut self) -> Task<Message> {
        let Some(key) = &self.key else {
            return self.show_error("Нет ключа шифрования, задачи не сохраняются");
        };

        match save_tasks(key, &self.tasks_path, &self.tasks) {
            Ok(()) => Task::none(),
            Err(e) => {
                eprintln!("{:#}", e);
                self.show_error("Не удалось сохранить задачи")
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input = value;
                Task::none()
            }
            Message::AddTask => {
                let description = self.input.trim().to_owned();
                if description.is_empty() {
                    return Task::none();
                }
                self.tasks.push(TodoItem {
                    id: self.next_id,
                    description,
                    done: false,
                });
                self.next_id += 1;
                self.input.clear();
                self.save()
            }
            Message::ToggleTask(id) => {
                if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                    task.done = !task.done;
                    self.save()
                } else {
                    Task::none()
                }
            }
            Message::DeleteTask(id) => {
                self.tasks.retain(|t| t.id != id);
                self.save()
            }
            Message::HideError(id) => {
                if id == self.error_id {
                    self.error = None;
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let list: Element<'_, Message> = if self.tasks.is_empty() {
            container(text("Задач нет").color(colors::GRAY))
                .center_x(Length::Fill)
                .into()
        } else {
            let tasks = column(self.tasks.iter().map(task_row)).spacing(spacing::MD);
            scrollable(tasks)
                .direction(Direction::Vertical(
                    Scrollbar::new()
                        .width(4)
                        .scroller_width(4)
                        .spacing(spacing::SM),
                ))
                .into()
        };
        let content = column![
            text_input("Введите описание задачи...", &self.input)
                .on_input(Message::InputChanged)
                .on_submit(Message::AddTask)
                .padding(spacing::MD)
                .style(theme::input::primary),
            list
        ]
        .spacing(spacing::XL)
        .padding(spacing::MD);

        let main = column![header(), content];

        match &self.error {
            Some(error) => stack![main, toast(error)].into(),
            None => main.into(),
        }
    }
}
