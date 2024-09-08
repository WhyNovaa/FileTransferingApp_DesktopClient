use iced::{window, Length, Padding, Size, Vector};
use iced::{Alignment, Background, Border, Element, Sandbox, Settings, Shadow};
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Theme;
use iced::widget::{button, container, TextInput, text, Button, Column, Container, Row, Scrollable, Space, Image};
use reqwest::blocking::Client;
use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use iced::Alignment::Center;
use iced::futures::future::err;
use iced::futures::TryFutureExt;
use serde_json;
use serde::{Deserialize, Serialize};


fn main() -> iced::Result {

    let window_settings = window::Settings {
        min_size: Some(Size::new(700.0, 600.0)),
        ..window::Settings::default()
    };

    App::run(Settings{
        window: window_settings,
        ..Settings::default()
    })
}


#[derive(Serialize, Deserialize, Debug)]
struct Group {
    id: i64,
    name: String,
    rights: String
}
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i64,
    login: String,
    hashed_password: String,
    salt: String,
    group: Group
}




struct App {
    theme: Theme,
    page: Page,
    login_field: LoginField,
    token: String,
    server_url: String,
    client: Client
}

struct LoginField {
    login: String,
    password: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page{
    Login,
    Main
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    LoginSubmit,
    LoginFieldChanged(String, String),
    DeleteFile(String),
    EditFile(String),
}



impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        dotenv().ok();
        Self {
            theme: Theme::Dark,
            page: Page::Login,
            login_field: LoginField {
                login: String::new(),
                password: String::new()
            },
            token: String::new(),
            server_url: env::var("SERVER_URL").expect("SERVER_URL must be set").to_string(),
            client: Client::new()
        }
    }

    fn title(&self) -> String {
        String::from("FTA")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                } else {
                    Theme::Light
                };
            }

            Message::LoginSubmit => {
                let mut params = HashMap::new();
                params.insert("username", "test1");
                params.insert("password", "test1");

                // Обработка ошибок вручную
                let result = self.client.post("http://127.0.0.1:8000/login")
                    .form(&params)
                    .send();

                match result {
                    Ok(response) => {
                        let json_result: Result<HashMap<String, String>, _> = response.json();
                        match json_result {
                            Ok(json) => println!("Response JSON: {:?}", json["token"]),
                            Err(e) => eprintln!("Error parsing JSON: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Error sending request: {}", e),
                }
            }

            Message::LoginFieldChanged(login, password) => {
                self.login_field.login = login;
                self.login_field.password = password;
            }

            Message::DeleteFile(_filename) => {
                // Реализация удаления файла
            }

            Message::EditFile(_filename) => {
                // Реализация редактирования файла
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let content = match self.page {
            Page::Login => log_in_page(&self.login_field),
            Page::Main => main_page(&self.client ,&self.token)

        };


        let wrapper =  Column::new();
        let wrapper = Scrollable::new(
            match self.page {
            Page::Login => wrapper.spacing(10)
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .push(content)
                .push(page_footer()),

            Page::Main => wrapper.push(page_footer())
                .spacing(10)
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .push(content),
        });

        let temp_container = container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .style(iced::theme::Container::Custom(Box::new(ContainerStyle)));

        let container = match self.page {
                Page::Login => temp_container.center_y(),
                Page::Main => temp_container.align_y(Vertical::Top),
        };
        container.width(Length::Fill).height(Length::Fill).into()
    }


}

/*async fn login_reqwest(client: &Client, server_url: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("username", "test1");
    params.insert("password", "test1");

    // Выполнение запроса и ожидание ответа
    let response = client
        .post(format!("{}/login", server_url))
        .form(&params)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    if let Some(token) = response.get("token") {
        println!("Значение для 'token': {}", token); // TEMP

        let data = client
            .get(format!("{}/admin/users", server_url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?
            .text()
            .await?;

        let users: Vec<User> = serde_json::from_str(&data)?;
        for user in users {
            println!("{}", user.login); // TEMP
        }
        Ok(String::from(token))
    } else {
        println!("Неправильный логин или пароль");
        Err(format!("Request failed").into())
    }
}

async fn check_login(client: &Client, server_url: &str) -> Result<String, String> {
    let mut params = HashMap::new();
    params.insert("username", "test1");
    params.insert("password", "test1");

    let response = client
        .post(format!("{}/login", server_url))
        .form(&params)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<HashMap<String, String>>()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(token) = response.get("token") {
        Ok(token.clone())
    } else {
        Err("No token received".into())
    }
}
async fn check_login1(client: &Client, server_url: &String) -> Option<String>{
    match login_reqwest(client, server_url).await {
        Ok(token) => {
            if token != ""{
                println!("Login successful!");
                token.into()
            } else {
                println!("Login failed: No token received.");
                token.into()
            }
        }
        Err(e) => None
    }
}*/
fn page_footer() -> Container<'static, Message> {
    let footer = Row::new()
        .push(
            button("Toggle Theme")
                .on_press(Message::ToggleTheme)
                .style(iced::theme::Button::Custom(Box::new(ButtonStyle::ThemeButton)),
            )
        )
        .align_items(Alignment::Center)
        .spacing(10);

    container(footer).center_x().center_y()

}
fn log_in_page(login_field: &LoginField) -> Container<Message> {
    let column = Column::new()
        .push(text("File Transferring App"))
        .push(
            input_field("Login", &login_field.login)
            .on_input(
                |login| {
                Message::LoginFieldChanged(login, login_field.password.clone())
                }
            )
        )
        .push(
            input_field("Password", &login_field.password)
            .on_input(
                |password| {
                    Message::LoginFieldChanged(login_field.login.clone(), password)
                }
            )
        )
        .push(submit_btn("Login", Message::LoginSubmit))
        .padding(Padding::from([50, 20]))
        .align_items(Alignment::Center)
        .spacing(40);

    container(column)
        .padding(Padding::from(20))
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
}

fn create_row() -> Container<'static, Message> {
    let row = Row::new()
        .push(Space::with_width(10))
        .push(text("text1").size(20))
        .push(Space::with_width(Length::Fill)) // Используем пробел для распределения пространства
        .push(edit_btn(Message::DeleteFile("TEMP".to_string())))
        .push(Space::with_width(20))
        .push(del_btn(Message::DeleteFile("TEMP".to_string())))
        .push(Space::with_width(10))
        .height(Length::Fill)
        .align_items(Alignment::Center);

    container(row)
        .style(iced::theme::Container::Custom(Box::new(FileStyle)))
}
fn main_page(client: &Client, token: &str) -> Container<'static, Message> {


    let mut column = Column::new()
        .width(Length::Fill)
        .spacing(30);

    column = column.push(Space::with_height(0));
    for i in 1..50 {

        column = column.push(
                Row::new().push(Space::with_width(20))
            .push(create_row())
            .push(Space::with_width(20))
            .height(40)
        );
    }
    column = column.push(Space::with_height(0));

    container(column)
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle)))
        .align_y(Vertical::Top)

}

fn input_field(_placeholder: &str, _value: &str, ) -> TextInput<'static, Message> {
    TextInput::new(_placeholder, _value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

fn del_btn(event: Message) -> Button<'static, Message> {
    let image = Image::new("src/resources/delete.png");

    Button::new(image)
        .on_press(event)
        .width(32)
        .height(32)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Transparent)))
}

fn edit_btn(event: Message) -> Button<'static, Message> {
    let image = Image::new("src/resources/edit.png");

    Button::new(image)
        .on_press(event)
        .width(32)
        .height(32)
        .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Transparent)))
}


fn submit_btn(name: &str, event: Message) -> Button<Message> {
    Button::new(
        text(name)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .size(21)
    )
    .on_press(event)
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(45.0))
    .style(iced::theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}



enum ButtonStyle {
    Standard,
    ThemeButton,
    Transparent, // Новый стиль
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, theme: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: match self {
                Self::Standard => Some(Background::Color(iced::Color::from_rgb(0.059, 0.463, 0.702))),
                Self::ThemeButton => Some(Background::Color(iced::Color::default())),
                Self::Transparent => None,
            },
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::ThemeButton => Border::default(),
                Self::Transparent => Border::default(),
            },
            shadow_offset: match self {
                Self::Standard => Vector::new(0.0, 2.0),
                Self::ThemeButton => Vector::new(0.0, 0.0),
                Self::Transparent => Vector::new(0.0, 0.0),
            },
            shadow: match self {
                Self::Standard => Shadow {
                    color: iced::Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::ThemeButton => Shadow::default(),
                Self::Transparent => Shadow::default(),
            },
            text_color: {
                if theme == &Theme::Light {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::BLACK,
                        Self::Transparent => iced::Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => iced::Color::WHITE,
                        Self::ThemeButton => iced::Color::WHITE,
                        Self::Transparent => iced::Color::WHITE,
                    }
                }
            },
        }
    }
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}

struct FileStyle;
impl container::StyleSheet for FileStyle {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(50),
            background: None,
            shadow: Shadow {
                color: iced::Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}