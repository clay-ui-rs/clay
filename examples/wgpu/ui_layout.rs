use clay_layout::{
    elements::FloatingAttachToElement,
    fixed, grow,
    layout::{Alignment, LayoutDirection::TopToBottom, Padding},
    math::Dimensions,
    percent,
    render_commands::RenderCommand,
    text::TextConfig,
    Clay, ClayLayoutScope, Color, Declaration,
};

const CLAY_ALIGN_Y_CENTER: Alignment = Alignment {
    x: clay_layout::layout::LayoutAlignmentX::Left,
    y: clay_layout::layout::LayoutAlignmentY::Center,
};

const WHITE: Color = Color::rgb(255.0, 255.0, 255.0);

trait CustomStyles<ImageElementData, CustomElementData> {
    fn layout_expand(&mut self) -> &mut Self;
    fn content_background_config(&mut self) -> &mut Self;
}

impl<ImageElementData, CustomElementData> CustomStyles<ImageElementData, CustomElementData>
    for Declaration<'_, ImageElementData, CustomElementData>
{
    fn layout_expand(&mut self) -> &mut Self {
        self.layout().width(grow!()).height(grow!()).end();

        self
    }

    fn content_background_config(&mut self) -> &mut Self {
        self.background_color(Color::rgb(90.0, 90.0, 90.0))
            .corner_radius()
            .all(8.0)
            .end();

        self
    }
}

fn render_header_button<'a, ImageElementData: 'a, CustomElementData: 'a>(
    clay: &ClayLayoutScope<'a, 'a, ImageElementData, CustomElementData>,
    text: &str,
) {
    clay.with(
        &Declaration::new()
            .layout()
            .padding(Padding::new(16, 16, 8, 8))
            .end()
            .background_color(Color::rgb(140.0, 140.0, 140.0))
            .corner_radius()
            .all(5.0)
            .end(),
        |clay| {
            clay.text(text, TextConfig::new().font_size(16).color(WHITE).end());
        },
    );
}

fn render_dropdown_menu_item<'a, ImageElementData: 'a, CustomElementData: 'a>(
    clay: &ClayLayoutScope<'a, 'a, ImageElementData, CustomElementData>,
    text: &str,
) {
    clay.with(
        &Declaration::new().layout().padding(Padding::all(16)).end(),
        |clay| {
            clay.text(text, TextConfig::new().font_size(16).color(WHITE).end());
        },
    );
}
pub struct Document {
    pub title: String,
    pub contents: String,
}

#[derive(Default)]
pub struct ClayState {
    pub documents: Vec<Document>,
    pub selected_document_index: u8,
    pub mouse_down_rising_edge: bool,
    pub mouse_position: (f32, f32),
    pub scroll_delta: (f32, f32),
    pub size: (f32, f32),
}

pub fn initialize_user_data(user_data: &mut ClayState) {
    user_data.documents
        .push(Document{
            title:"Squirrels".to_string(), 
            contents: "The Secret Life of Squirrels: Nature's Clever Acrobats\n\"Squirrels are often overlooked creatures, dismissed as mere park inhabitants or backyard nuisances. Yet, beneath their fluffy tails and twitching noses lies an intricate world of cunning, agility, and survival tactics that are nothing short of fascinating. As one of the most common mammals in North America, squirrels have adapted to a wide range of environments from bustling urban centers to tranquil forests and have developed a variety of unique behaviors that continue to intrigue scientists and nature enthusiasts alike.\n\"\n\"Master Tree Climbers\n\"At the heart of a squirrel's skill set is its impressive ability to navigate trees with ease. Whether they're darting from branch to branch or leaping across wide gaps, squirrels possess an innate talent for acrobatics. Their powerful hind legs, which are longer than their front legs, give them remarkable jumping power. With a tail that acts as a counterbalance, squirrels can leap distances of up to ten times the length of their body, making them some of the best aerial acrobats in the animal kingdom.\n\"But it's not just their agility that makes them exceptional climbers. Squirrels' sharp, curved claws allow them to grip tree bark with precision, while the soft pads on their feet provide traction on slippery surfaces. Their ability to run at high speeds and scale vertical trunks with ease is a testament to the evolutionary adaptations that have made them so successful in their arboreal habitats.\n\"\n\"Food Hoarders Extraordinaire\n\"Squirrels are often seen frantically gathering nuts, seeds, and even fungi in preparation for winter. While this behavior may seem like instinctual hoarding, it is actually a survival strategy that has been honed over millions of years. Known as \"scatter hoarding,\" squirrels store their food in a variety of hidden locations, often burying it deep in the soil or stashing it in hollowed-out tree trunks.\nInterestingly, squirrels have an incredible memory for the locations of their caches. Research has shown that they can remember thousands of hiding spots, often returning to them months later when food is scarce. However, they don't always recover every stash some forgotten caches eventually sprout into new trees, contributing to forest regeneration. This unintentional role as forest gardeners highlights the ecological importance of squirrels in their ecosystems.\n\nThe Great Squirrel Debate: Urban vs. Wild\nWhile squirrels are most commonly associated with rural or wooded areas, their adaptability has allowed them to thrive in urban environments as well. In cities, squirrels have become adept at finding food sources in places like parks, streets, and even garbage cans. However, their urban counterparts face unique challenges, including traffic, predators, and the lack of natural shelters. Despite these obstacles, squirrels in urban areas are often observed using human infrastructure such as buildings, bridges, and power lines as highways for their acrobatic escapades.\nThere is, however, a growing concern regarding the impact of urban life on squirrel populations. Pollution, deforestation, and the loss of natural habitats are making it more difficult for squirrels to find adequate food and shelter. As a result, conservationists are focusing on creating squirrel-friendly spaces within cities, with the goal of ensuring these resourceful creatures continue to thrive in both rural and urban landscapes.\n\nA Symbol of Resilience\nIn many cultures, squirrels are symbols of resourcefulness, adaptability, and preparation. Their ability to thrive in a variety of environments while navigating challenges with agility and grace serves as a reminder of the resilience inherent in nature. Whether you encounter them in a quiet forest, a city park, or your own backyard, squirrels are creatures that never fail to amaze with their endless energy and ingenuity.\nIn the end, squirrels may be small, but they are mighty in their ability to survive and thrive in a world that is constantly changing. So next time you spot one hopping across a branch or darting across your lawn, take a moment to appreciate the remarkable acrobat at work a true marvel of the natural world.\n".to_string()
        });
    user_data.documents
        .push(Document{
            title:"Lorem Ipsum".to_string(), 
            contents: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string()
        });
}

pub fn create_layout<'render>(
    clay: &'render mut Clay,
    user_data: &mut ClayState,
    time_delta: f32,
) -> impl Iterator<Item = RenderCommand<'render, (), ()>> {
    clay.set_layout_dimensions(user_data.size.into());
    clay.pointer_state(user_data.mouse_position.into(), false);
    clay.update_scroll_containers(false, user_data.scroll_delta.into(), time_delta);

    let clay = clay.begin::<(), ()>();

    clay.with(
        &Declaration::new()
            .layout()
            .width(grow!())
            .height(grow!())
            .end()
            .id(clay.id("outer_container"))
            .layout()
            .direction(TopToBottom)
            .padding(Padding::all(16))
            .child_gap(16)
            .end()
            .background_color(Color::rgb(43.0, 41.0, 51.0)),
        |clay| {
            clay.with(
                &Declaration::new()
                    .content_background_config()
                    .id(clay.id("header_bar"))
                    .layout()
                    .width(grow!())
                    .height(fixed!(120.0))
                    .padding(Padding {
                        left: 16,
                        right: 16,
                        top: 8,
                        bottom: 8,
                    })
                    .child_gap(16)
                    .child_alignment(CLAY_ALIGN_Y_CENTER)
                    .end(),
                |clay| {
                    clay.with(
                        &Declaration::new()
                            .id(clay.id("file_button"))
                            .layout()
                            .padding(Padding {
                                left: 16,
                                right: 16,
                                top: 8,
                                bottom: 8,
                            })
                            .end()
                            .background_color(Color::rgb(140.0, 140.0, 140.0))
                            .corner_radius()
                            .all(5.0)
                            .end(),
                        |clay| {
                            clay.text("File", TextConfig::new().font_size(16).color(WHITE).end());

                            let file_menu_visible = clay.pointer_over(clay.id("file_button"))
                                || clay.pointer_over(clay.id("file_menu"));

                            if file_menu_visible {
                                clay.with(
                                    &Declaration::new()
                                        .id(clay.id("file_menu"))
                                        .floating()
                                        .attach_to(FloatingAttachToElement::Parent)
                                        .end()
                                        .layout()
                                        .padding(Padding::new(0, 0, 8, 8))
                                        .end(),
                                    |clay| {
                                        clay.with(
                                            &Declaration::new()
                                                .layout()
                                                .direction(TopToBottom)
                                                .width(fixed!(200.0))
                                                .end()
                                                .background_color(Color::rgb(40.0, 40.0, 40.0))
                                                .corner_radius()
                                                .all(8.0)
                                                .end(),
                                            |clay| {
                                                render_dropdown_menu_item(clay, "New");
                                                render_dropdown_menu_item(clay, "Open");
                                                render_dropdown_menu_item(clay, "Close");
                                            },
                                        );
                                    },
                                );
                            }
                        },
                    );

                    render_header_button(clay, "Edit");
                    clay.with(&Declaration::new().layout().width(grow!()).end(), |_| {});
                    render_header_button(clay, "Upload");
                    render_header_button(clay, "Media");
                    render_header_button(clay, "Support");
                },
            );

            clay.with(
                &Declaration::new()
                    .layout_expand()
                    .id(clay.id("lower_content"))
                    .layout()
                    .child_gap(16)
                    .end(),
                |clay| {
                    clay.with(
                        &Declaration::new()
                            .content_background_config()
                            .id(clay.id("sidebar"))
                            .layout()
                            .direction(TopToBottom)
                            .padding(Padding::all(16))
                            .child_gap(8)
                            .width(percent!(0.25))
                            .height(grow!())
                            .end(),
                        |clay| {
                            for i in 0..user_data.documents.len() {
                                let document = user_data.documents.get_mut(i).unwrap();
                                let mut side_bar_button_layout: Declaration<'_, (), ()> =
                                    Declaration::new()
                                        .layout()
                                        .width(grow!())
                                        .padding(Padding::all(16))
                                        .end()
                                        .to_owned();

                                if i as u8 == user_data.selected_document_index {
                                    clay.with_styling(
                                        |clay| {
                                            if clay.hovered() {
                                                if user_data.mouse_down_rising_edge {
                                                    user_data.selected_document_index = i as u8;
                                                }

                                                *side_bar_button_layout
                                                    .background_color(Color::rgb(
                                                        120.0, 120.0, 120.0,
                                                    ))
                                                    .corner_radius()
                                                    .all(8.0)
                                                    .end()
                                                    .border()
                                                    .all_directions(3)
                                                    .color(WHITE)
                                                    .end()
                                            } else {
                                                *side_bar_button_layout
                                                    .background_color(Color::rgb(
                                                        120.0, 120.0, 120.0,
                                                    ))
                                                    .corner_radius()
                                                    .all(8.0)
                                                    .end()
                                            }
                                        },
                                        |clay| {
                                            clay.text(
                                                &document.title,
                                                TextConfig::new().font_size(20).color(WHITE).end(),
                                            );
                                        },
                                    );
                                } else {
                                    clay.with_styling(
                                        |clay| {
                                            if clay.hovered() {
                                                if user_data.mouse_down_rising_edge {
                                                    user_data.selected_document_index = i as u8;
                                                }

                                                *side_bar_button_layout
                                                    .border()
                                                    .all_directions(3)
                                                    .color(WHITE)
                                                    .end()
                                            } else {
                                                side_bar_button_layout
                                            }
                                        },
                                        |clay| {
                                            clay.text(
                                                &document.title,
                                                TextConfig::new().font_size(20).color(WHITE).end(),
                                            );
                                        },
                                    );
                                }
                            }
                        },
                    );

                    clay.with(
                        Declaration::new()
                            .content_background_config()
                            .layout_expand()
                            .id(clay.id("main_content"))
                            .scroll(false, true)
                            .layout()
                            .direction(TopToBottom)
                            .child_gap(16)
                            .padding(Padding::all(16))
                            .end(),
                        |clay| {
                            let selected_documtent =
                                &user_data.documents[user_data.selected_document_index as usize];
                            clay.text(
                                &selected_documtent.title,
                                TextConfig::new().font_size(24).color(WHITE).end(),
                            );

                            clay.text(
                                &selected_documtent.contents,
                                TextConfig::new().font_size(24).color(WHITE).end(),
                            );
                        },
                    );
                },
            );
        },
    );

    clay.end()
}

use crate::UIState;
use std::{cell::RefCell, rc::Rc};
pub fn measure_text(text: &str, config: &TextConfig, ui: &mut Rc<RefCell<UIState>>) -> Dimensions {
    ui.borrow_mut()
        .measure_text(text, config.font_size as f32, config.line_height as f32)
}
