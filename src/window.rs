use fltk::{
    app, draw::set_cursor, enums::{Color, Event, Key}, prelude::*, text::{TextBuffer, TextEditor}, window::Window
};


use crate::command;

pub struct Settings {
    prompt: String,
    terminal_name: String,
    terminal_default_width: i32,
    terminal_default_height: i32,
}

impl Settings {
    pub fn init(prompt: &str, terminal_name: &str, terminal_default_width: i32, terminal_default_height: i32) -> Self {
        let prompt = prompt.to_string();
        let terminal_name = terminal_name.to_string();

        Settings {
            prompt,
            terminal_name,
            terminal_default_width,
            terminal_default_height,
        }
    }
}


fn apply_settings(config: &Settings) -> (TextEditor,Window){
    let wind = Window::new(1920 / 2 - config.terminal_default_width / 2, 1080 / 2 - config.terminal_default_height / 2, config.terminal_default_width, config.terminal_default_height, config.terminal_name.as_str());


    let mut editor = TextEditor::new(0, 0,config.terminal_default_width, config.terminal_default_height, "");
    editor.set_cursor_style(fltk::text::Cursor::Simple);
    editor.set_cursor_color(Color::White);
    editor.set_color(Color::from_rgb(22, 25, 29));
    editor.set_text_color(Color::White);

    return (editor, wind);

}


pub fn create(config: Settings) {
    let app = app::App::default();
    let mut buffer = TextBuffer::default();

    let (mut editor, mut window) = apply_settings(&config);

    editor.set_buffer(buffer.clone());

    buffer.append(&config.prompt);
    editor.set_insert_position(buffer.length());

    window.resizable(&editor);

    let mut last_prompt_idx = config.prompt.len() as i32;

    let mut oldBuffer = buffer.clone();

    let mut lines: Vec<TextBuffer> = Vec::new();

    let mut line_above = TextBuffer::default();


    let mut up_count = 0;

    editor.handle({
        let mut buffer = buffer.clone();
        move |widget, event| {
            match event {
                Event::KeyDown => {
                    let key = app::event_key();
                    println!("last prompt index: {}", last_prompt_idx);
                    if key == Key::BackSpace {
                        if widget.insert_position() <= last_prompt_idx{
                            println!("User versucht den Prompt zu löschen");

                            buffer.append(" ");
                            widget.set_insert_position(last_prompt_idx + 1 as i32);
                        }
                    }
                    if key == Key::Up {
                        // prevent the user from going up
                        buffer = oldBuffer.clone();

                        let mut idx = (lines.len() - 1 - up_count) as i32;

                        if idx < 0 {
                            idx = 0;
                        }


                        up_count += 1;

                        println!("index: {}" ,idx);

                        let wanted_line = lines[idx as usize].clone();

                        println!("Wanted line text: {}", wanted_line.text());

                        for line in lines.iter() {
                            println!("Line: {}", line.text());
                        }

                        buffer.set_text(&(String::new() + &buffer.text().as_str().to_owned() + &wanted_line.text().as_str().to_owned()));
                        widget.set_insert_position(buffer.length());

                    }
                    if key == Key::Enter {
                        if let Some(buf) = widget.buffer() {
                            let content = buf.text();
                            if let Some(last_line) = content.lines().last() {
                                if last_line.starts_with(&config.prompt) {
                                    let user_input = last_line.trim_start_matches(&config.prompt);

                                    line_above.set_text(user_input);

                                    println!("Line above var: {}" , line_above.text());

                                    lines.push(line_above.clone());

                                    println!("Command: {}", user_input);

                                    up_count = 0;

                                    command::handle(user_input);

                                    // prompt position muss auch geupdatet werden
                                    last_prompt_idx += user_input.len() as i32 + config.prompt.len() as i32;

                                    if user_input.len() == 0 {
                                        // wenn man nur enter drückt ohne input muss der prompt index trotzdem um 1 erhöht werden
                                        // obowhl der user input die länge 0 hat
                                        last_prompt_idx += 1;
                                    }

                                    buffer.append(&config.prompt);
                                    widget.set_insert_position(buf.length());
                                    oldBuffer = buffer.clone();
                                }
                            }
                        }
                        return true;
                    }
                    false
                },
                _ => false,
            }
        }
    });


    window.end();
    window.show();
    app.run().unwrap();
}