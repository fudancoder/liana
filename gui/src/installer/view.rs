use iced::widget::{
    scrollable::Properties, Button, Checkbox, Column, Container, PickList, Row, Scrollable, Space,
};
use iced::{alignment, Alignment, Element, Length};

use liana::miniscript::bitcoin;

use crate::{
    hw::HardwareWallet,
    installer::{
        message::{self, Message},
        step::Context,
        Error,
    },
    ui::{
        color,
        component::{
            button, card, collapse, container, form, separation,
            text::{text, Text},
            tooltip,
        },
        icon,
        util::Collection,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Network {
    Mainnet,
    Testnet,
    Regtest,
    Signet,
}

impl From<bitcoin::Network> for Network {
    fn from(n: bitcoin::Network) -> Self {
        match n {
            bitcoin::Network::Bitcoin => Network::Mainnet,
            bitcoin::Network::Testnet => Network::Testnet,
            bitcoin::Network::Regtest => Network::Regtest,
            bitcoin::Network::Signet => Network::Signet,
        }
    }
}

impl From<Network> for bitcoin::Network {
    fn from(network: Network) -> bitcoin::Network {
        match network {
            Network::Mainnet => bitcoin::Network::Bitcoin,
            Network::Testnet => bitcoin::Network::Testnet,
            Network::Regtest => bitcoin::Network::Regtest,
            Network::Signet => bitcoin::Network::Signet,
        }
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "Bitcoin mainnet"),
            Self::Testnet => write!(f, "Bitcoin testnet"),
            Self::Regtest => write!(f, "Bitcoin regtest"),
            Self::Signet => write!(f, "Bitcoin signet"),
        }
    }
}

const NETWORKS: [Network; 4] = [
    Network::Mainnet,
    Network::Testnet,
    Network::Signet,
    Network::Regtest,
];

pub fn welcome<'a>() -> Element<'a, Message> {
    Container::new(Container::new(
        Column::new()
            .push(
                Row::new()
                    .spacing(20)
                    .push(
                        Button::new(
                            Container::new(
                                Column::new()
                                    .width(Length::Units(200))
                                    .push(icon::wallet_icon().size(50).width(Length::Units(100)))
                                    .push(text("Create new wallet"))
                                    .align_items(Alignment::Center),
                            )
                            .padding(50),
                        )
                        .style(button::Style::Border.into())
                        .on_press(Message::CreateWallet),
                    )
                    .push(
                        Button::new(
                            Container::new(
                                Column::new()
                                    .width(Length::Units(200))
                                    .push(icon::import_icon().size(50).width(Length::Units(100)))
                                    .push(text("Import wallet"))
                                    .align_items(Alignment::Center),
                            )
                            .padding(50),
                        )
                        .style(button::Style::Border.into())
                        .on_press(Message::ImportWallet),
                    ),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center),
    ))
    .center_y()
    .center_x()
    .height(Length::Fill)
    .width(Length::Fill)
    .into()
}

#[allow(clippy::too_many_arguments)]
pub fn define_descriptor<'a>(
    progress: (usize, usize),
    network: bitcoin::Network,
    network_valid: bool,
    spending_keys: Vec<Element<'a, Message>>,
    recovery_keys: Vec<Element<'a, Message>>,
    sequence: &form::Value<String>,
    spending_threshold: usize,
    recovery_threshold: usize,
    valid: bool,
    error: Option<&String>,
) -> Element<'a, Message> {
    let row_network = Row::new()
        .spacing(10)
        .align_items(Alignment::Center)
        .push(text("Network:").bold())
        .push(Container::new(
            PickList::new(&NETWORKS[..], Some(Network::from(network)), |net| {
                Message::Network(net.into())
            })
            .padding(10),
        ))
        .push_maybe(if network_valid {
            None
        } else {
            Some(card::warning(
                "A data directory already exists for this network".to_string(),
            ))
        });

    let col_spending_keys = Column::new()
        .push(
            Row::new()
                .spacing(10)
                .push(text("Primary path:").bold())
                .push(tooltip(
                    super::prompt::DEFINE_DESCRIPTOR_PRIMATRY_PATH_TOOLTIP,
                )),
        )
        .push(separation().width(Length::Fill))
        .push(
            Container::new(
                Row::new()
                    .align_items(Alignment::Center)
                    .push_maybe(if spending_keys.len() > 1 {
                        Some(threshsold_input::threshsold_input(
                            spending_threshold,
                            spending_keys.len(),
                            |value| {
                                Message::DefineDescriptor(
                                    message::DefineDescriptor::ThresholdEdited(false, value),
                                )
                            },
                        ))
                    } else {
                        None
                    })
                    .push(
                        Scrollable::new(
                            Row::new()
                                .spacing(5)
                                .align_items(Alignment::Center)
                                .push(Row::with_children(spending_keys).spacing(5))
                                .push(
                                    Button::new(
                                        Container::new(icon::plus_icon().size(50))
                                            .width(Length::Units(250))
                                            .height(Length::Units(250))
                                            .align_y(alignment::Vertical::Center)
                                            .align_x(alignment::Horizontal::Center),
                                    )
                                    .width(Length::Units(250))
                                    .height(Length::Units(250))
                                    .style(button::Style::TransparentBorder.into())
                                    .on_press(
                                        Message::DefineDescriptor(
                                            message::DefineDescriptor::AddKey(false),
                                        ),
                                    ),
                                )
                                .padding(5),
                        )
                        .horizontal_scroll(Properties::new().width(3).scroller_width(3)),
                    ),
            )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        )
        .spacing(10);

    let col_recovery_keys = Column::new()
        .push(text("Recovery path:").bold())
        .push(separation().width(Length::Fill))
        .push(
            Container::new(
                Row::new()
                    .align_items(Alignment::Center)
                    .push_maybe(if recovery_keys.len() > 1 {
                        Some(threshsold_input::threshsold_input(
                            recovery_threshold,
                            recovery_keys.len(),
                            |value| {
                                Message::DefineDescriptor(
                                    message::DefineDescriptor::ThresholdEdited(true, value),
                                )
                            },
                        ))
                    } else {
                        None
                    })
                    .push(
                        Scrollable::new(
                            Row::new()
                                .spacing(5)
                                .align_items(Alignment::Center)
                                .push(Row::with_children(recovery_keys).spacing(5))
                                .push(
                                    Button::new(
                                        Container::new(icon::plus_icon().size(50))
                                            .width(Length::Units(250))
                                            .height(Length::Units(250))
                                            .align_y(alignment::Vertical::Center)
                                            .align_x(alignment::Horizontal::Center),
                                    )
                                    .width(Length::Units(250))
                                    .height(Length::Units(250))
                                    .style(button::Style::TransparentBorder.into())
                                    .on_press(
                                        Message::DefineDescriptor(
                                            message::DefineDescriptor::AddKey(true),
                                        ),
                                    ),
                                )
                                .padding(5),
                        )
                        .horizontal_scroll(Properties::new().width(3).scroller_width(3)),
                    ),
            )
            .width(Length::Fill)
            .align_x(alignment::Horizontal::Center),
        )
        .spacing(10);

    let col_sequence = Container::new(
        Row::new()
            .spacing(50)
            .align_items(Alignment::Center)
            .push(Container::new(icon::arrow_down().size(50)).align_x(alignment::Horizontal::Right))
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .spacing(10)
                            .push(text("Blocks before recovery:").bold())
                            .push(tooltip(super::prompt::DEFINE_DESCRIPTOR_SEQUENCE_TOOLTIP)),
                    )
                    .push(
                        Container::new(
                            form::Form::new("Number of block", sequence, |msg| {
                                Message::DefineDescriptor(
                                    message::DefineDescriptor::SequenceEdited(msg),
                                )
                            })
                            .warning("Please enter correct block number")
                            .size(20)
                            .padding(10),
                        )
                        .width(Length::Units(150)),
                    )
                    .spacing(10),
            )
            .padding(20),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center);

    layout(
        progress,
        Column::new()
            .push(text("Create the wallet").bold().size(50))
            .push(
                Column::new()
                    .push(row_network)
                    .push(col_spending_keys)
                    .push(col_sequence)
                    .push(col_recovery_keys)
                    .spacing(25),
            )
            .push(if !valid {
                button::primary(None, "Next").width(Length::Units(200))
            } else {
                button::primary(None, "Next")
                    .width(Length::Units(200))
                    .on_press(Message::Next)
            })
            .push_maybe(error.map(|e| card::error("Failed to create descriptor", e.to_string())))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center),
    )
}

pub fn import_descriptor<'a>(
    progress: (usize, usize),
    network: bitcoin::Network,
    network_valid: bool,
    imported_descriptor: &form::Value<String>,
    error: Option<&String>,
) -> Element<'a, Message> {
    let row_network = Row::new()
        .spacing(10)
        .align_items(Alignment::Center)
        .push(text("Network:").bold())
        .push(Container::new(
            PickList::new(&NETWORKS[..], Some(Network::from(network)), |net| {
                Message::Network(net.into())
            })
            .padding(10),
        ))
        .push_maybe(if network_valid {
            None
        } else {
            Some(card::warning(
                "A data directory already exists for this network".to_string(),
            ))
        });
    let col_descriptor = Column::new()
        .push(text("Descriptor:").bold())
        .push(
            form::Form::new("Descriptor", imported_descriptor, |msg| {
                Message::DefineDescriptor(message::DefineDescriptor::ImportDescriptor(msg))
            })
            .warning("Please enter correct descriptor")
            .size(20)
            .padding(10),
        )
        .spacing(10);
    layout(
        progress,
        Column::new()
            .push(text("Import the wallet").bold().size(50))
            .push(
                Column::new()
                    .spacing(20)
                    .push(row_network)
                    .push(col_descriptor),
            )
            .push(if imported_descriptor.value.is_empty() {
                button::primary(None, "Next").width(Length::Units(200))
            } else {
                button::primary(None, "Next")
                    .width(Length::Units(200))
                    .on_press(Message::Next)
            })
            .push_maybe(error.map(|e| card::error("Invalid descriptor", e.to_string())))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center),
    )
}

pub fn register_descriptor<'a>(
    progress: (usize, usize),
    descriptor: String,
    hws: &[(HardwareWallet, Option<[u8; 32]>, bool)],
    error: Option<&Error>,
    processing: bool,
    chosen_hw: Option<usize>,
) -> Element<'a, Message> {
    layout(
        progress,
        Column::new()
            .push(text("Register descriptor").bold().size(50))
            .push(card::simple(
                Column::new()
                    .push(text("The descriptor:").small().bold())
                    .push(text(descriptor.clone()).small())
                    .push(
                        Row::new().push(Column::new().width(Length::Fill)).push(
                            button::transparent_border(Some(icon::clipboard_icon()), "Copy")
                                .on_press(Message::Clibpboard(descriptor)),
                        ),
                    )
                    .spacing(10)
                    .max_width(1000),
            ))
            .push_maybe(error.map(|e| card::error("Failed to register descriptor", e.to_string())))
            .push(
                Column::new()
                    .push(
                        Row::new()
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(
                                Container::new(
                                    text(format!("{} hardware wallets connected", hws.len()))
                                        .bold(),
                                )
                                .width(Length::Fill),
                            )
                            .push(
                                button::border(Some(icon::reload_icon()), "Refresh")
                                    .on_press(Message::Reload),
                            ),
                    )
                    .spacing(10)
                    .push(
                        hws.iter()
                            .enumerate()
                            .fold(Column::new().spacing(10), |col, (i, hw)| {
                                col.push(hw_list_view(
                                    i,
                                    &hw.0,
                                    Some(i) == chosen_hw,
                                    processing,
                                    hw.2,
                                ))
                            }),
                    )
                    .width(Length::Fill),
            )
            .push(if processing {
                button::primary(None, "Next").width(Length::Units(200))
            } else {
                button::primary(None, "Next")
                    .on_press(Message::Next)
                    .width(Length::Units(200))
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center),
    )
}

pub fn backup_descriptor<'a>(
    progress: (usize, usize),
    descriptor: String,
    done: bool,
) -> Element<'a, Message> {
    layout(
        progress,
        Column::new()
            .push(
                text("Did you backup your wallet descriptor ?")
                    .bold()
                    .size(50),
            )
            .push(
                Column::new()
                    .push(text(super::prompt::BACKUP_DESCRIPTOR_MESSAGE))
                    .push(collapse::Collapse::new(
                        || {
                            Button::new(
                                Row::new()
                                    .align_items(Alignment::Center)
                                    .spacing(10)
                                    .push(text("Learn more").small().bold())
                                    .push(icon::collapse_icon()),
                            )
                            .style(button::Style::Transparent.into())
                        },
                        || {
                            Button::new(
                                Row::new()
                                    .align_items(Alignment::Center)
                                    .spacing(10)
                                    .push(text("Learn more").small().bold())
                                    .push(icon::collapsed_icon()),
                            )
                            .style(button::Style::Transparent.into())
                        },
                        help_backup,
                    ))
                    .max_width(1000),
            )
            .push(card::simple(
                Column::new()
                    .push(text("The descriptor:").small().bold())
                    .push(text(descriptor.clone()).small())
                    .push(
                        Row::new().push(Column::new().width(Length::Fill)).push(
                            button::transparent_border(Some(icon::clipboard_icon()), "Copy")
                                .on_press(Message::Clibpboard(descriptor)),
                        ),
                    )
                    .spacing(10)
                    .max_width(1000),
            ))
            .push(Checkbox::new(
                "I have backed up my descriptor",
                done,
                Message::BackupDone,
            ))
            .push(if done {
                button::primary(None, "Next")
                    .on_press(Message::Next)
                    .width(Length::Units(200))
            } else {
                button::primary(None, "Next").width(Length::Units(200))
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center),
    )
}

pub fn help_backup<'a>() -> Element<'a, Message> {
    text(super::prompt::BACKUP_DESCRIPTOR_HELP).small().into()
}

pub fn define_bitcoin<'a>(
    progress: (usize, usize),
    address: &form::Value<String>,
    cookie_path: &form::Value<String>,
) -> Element<'a, Message> {
    let col_address = Column::new()
        .push(text("Address:").bold())
        .push(
            form::Form::new("Address", address, |msg| {
                Message::DefineBitcoind(message::DefineBitcoind::AddressEdited(msg))
            })
            .warning("Please enter correct address")
            .size(20)
            .padding(10),
        )
        .spacing(10);

    let col_cookie = Column::new()
        .push(text("Cookie path:").bold())
        .push(
            form::Form::new("Cookie path", cookie_path, |msg| {
                Message::DefineBitcoind(message::DefineBitcoind::CookiePathEdited(msg))
            })
            .warning("Please enter correct path")
            .size(20)
            .padding(10),
        )
        .spacing(10);

    layout(
        progress,
        Column::new()
            .push(
                text("Set up connection to the Bitcoin full node")
                    .bold()
                    .size(50),
            )
            .push(col_address)
            .push(col_cookie)
            .push(
                button::primary(None, "Next")
                    .on_press(Message::Next)
                    .width(Length::Units(200)),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(100)
            .spacing(50)
            .align_items(Alignment::Center),
    )
}

pub fn install<'a>(
    progress: (usize, usize),
    context: &Context,
    descriptor: String,
    generating: bool,
    config_path: Option<&std::path::PathBuf>,
    warning: Option<&'a String>,
) -> Element<'a, Message> {
    let mut col = Column::new()
        .push(
            Container::new(
                Column::new()
                    .spacing(10)
                    .push(
                        card::simple(
                            Column::new()
                                .spacing(5)
                                .push(text("Descriptor:").small().bold())
                                .push(text(descriptor).small()),
                        )
                        .width(Length::Fill),
                    )
                    .push(
                        card::simple(
                            Column::new()
                                .spacing(5)
                                .push(text("Hardware devices:").small().bold())
                                .push(context.hws.iter().fold(Column::new(), |acc, hw| {
                                    acc.push(
                                        Row::new()
                                            .spacing(5)
                                            .push(text(hw.0.to_string()).small())
                                            .push(text(format!("(fingerprint: {})", hw.1)).small()),
                                    )
                                })),
                        )
                        .width(Length::Fill),
                    )
                    .push(
                        card::simple(
                            Column::new()
                                .push(text("Bitcoind:").small().bold())
                                .push(
                                    Row::new()
                                        .spacing(5)
                                        .align_items(Alignment::Center)
                                        .push(text("Cookie path:").small())
                                        .push(
                                            text(format!(
                                                "{}",
                                                context
                                                    .bitcoind_config
                                                    .as_ref()
                                                    .unwrap()
                                                    .cookie_path
                                                    .to_string_lossy()
                                            ))
                                            .small(),
                                        ),
                                )
                                .push(
                                    Row::new()
                                        .spacing(5)
                                        .align_items(Alignment::Center)
                                        .push(text("Address:").small())
                                        .push(
                                            text(format!(
                                                "{}",
                                                context.bitcoind_config.as_ref().unwrap().addr
                                            ))
                                            .small(),
                                        ),
                                ),
                        )
                        .width(Length::Fill),
                    ),
            )
            .padding(50)
            .max_width(1000),
        )
        .spacing(50)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center);

    if let Some(error) = warning {
        col = col.push(text(error));
    }

    if generating {
        col = col.push(button::primary(None, "Installing ...").width(Length::Units(200)))
    } else if let Some(path) = config_path {
        col = col.push(
            Container::new(
                Column::new()
                    .push(Container::new(text("Installed !")))
                    .push(Container::new(
                        button::primary(None, "Start")
                            .on_press(Message::Exit(path.clone()))
                            .width(Length::Units(200)),
                    ))
                    .align_items(Alignment::Center)
                    .spacing(20),
            )
            .padding(50)
            .width(Length::Fill)
            .center_x(),
        );
    } else {
        col = col.push(
            button::primary(None, "Finalize installation")
                .on_press(Message::Install)
                .width(Length::Units(200)),
        );
    }

    layout(progress, col)
}

pub fn undefined_descriptor_key<'a>() -> Element<'a, message::DefineKey> {
    card::simple(
        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .align_items(Alignment::Center)
                    .push(icon::key_icon())
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Button::new(icon::cross_icon())
                            .style(button::Style::Transparent.into())
                            .on_press(message::DefineKey::Delete),
                    ),
            )
            .push(
                Container::new(
                    Column::new()
                        .spacing(5)
                        .push(
                            button::border(Some(icon::import_icon()), "from text input")
                                .on_press(message::DefineKey::ImportFromClipboard),
                        )
                        .push(
                            button::border(Some(icon::chip_icon()), "from hardware")
                                .on_press(message::DefineKey::ImportFromHardware),
                        ),
                )
                .height(Length::Fill)
                .align_y(alignment::Vertical::Center),
            ),
    )
    .padding(5)
    .height(Length::Units(250))
    .width(Length::Units(250))
    .into()
}

pub fn defined_descriptor_key<'a>(
    key: String,
    valid: bool,
    duplicate: bool,
) -> Element<'a, message::DefineKey> {
    let col = Column::new()
        .spacing(40)
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .push(
            Row::new()
                .align_items(Alignment::Center)
                .push(icon::key_icon())
                .push(Space::with_width(Length::Fill))
                .push(
                    Button::new(icon::cross_icon())
                        .style(button::Style::Transparent.into())
                        .on_press(message::DefineKey::Delete),
                ),
        )
        .push(
            Column::new()
                .align_items(Alignment::Center)
                .spacing(5)
                .push(
                    Container::new(
                        Scrollable::new(Container::new(text(key.clone())))
                            .height(Length::Units(50))
                            .horizontal_scroll(Properties::new().width(2).scroller_width(2)),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill),
                )
                .push(
                    button::transparent_border(Some(icon::clipboard_icon()), "Copy")
                        .on_press(message::DefineKey::Clipboard(key)),
                ),
        );

    if !valid {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                card::invalid(col)
                    .padding(5)
                    .height(Length::Units(250))
                    .width(Length::Units(250)),
            )
            .push(
                text("Key is for a different network")
                    .small()
                    .style(color::ALERT),
            )
            .into()
    } else if duplicate {
        Column::new()
            .align_items(Alignment::Center)
            .push(
                card::invalid(col)
                    .padding(5)
                    .height(Length::Units(250))
                    .width(Length::Units(250)),
            )
            .push(text("Key is a duplicate").small().style(color::ALERT))
            .into()
    } else {
        card::simple(col)
            .padding(5)
            .height(Length::Units(250))
            .width(Length::Units(250))
            .into()
    }
}

pub fn hardware_wallet_xpubs_modal<'a>(
    is_heir: bool,
    hws: &[HardwareWallet],
    error: Option<&Error>,
    processing: bool,
    chosen_hw: Option<usize>,
) -> Element<'a, Message> {
    card::simple(
        Column::new()
            .spacing(20)
            .push(
                text(if is_heir {
                    "Import the recovery public key:"
                } else {
                    "Import the user public key:"
                })
                .bold(),
            )
            .push(separation().width(Length::Fill))
            .push_maybe(error.map(|e| card::error("Failed to import xpub", e.to_string())))
            .push(if !hws.is_empty() {
                Column::new()
                    .push(
                        Row::new()
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(
                                Container::new(
                                    text(format!("{} hardware wallets connected", hws.len()))
                                        .bold(),
                                )
                                .width(Length::Fill),
                            )
                            .push(
                                button::border(Some(icon::reload_icon()), "Refresh")
                                    .on_press(Message::Reload),
                            ),
                    )
                    .spacing(10)
                    .push(
                        hws.iter()
                            .enumerate()
                            .fold(Column::new().spacing(10), |col, (i, hw)| {
                                col.push(hw_list_view(
                                    i,
                                    hw,
                                    Some(i) == chosen_hw,
                                    processing,
                                    false,
                                ))
                            }),
                    )
                    .width(Length::Fill)
            } else {
                Column::new()
                    .push(
                        Column::new()
                            .spacing(15)
                            .width(Length::Fill)
                            .push("Please connect a hardware wallet")
                            .push(button::border(None, "Refresh").on_press(Message::Reload))
                            .align_items(Alignment::Center),
                    )
                    .width(Length::Fill)
            })
            .width(Length::Units(600)),
    )
    .into()
}
pub fn clipboard_xpub_modal<'a>(
    form_xpub: &form::Value<String>,
    network: bitcoin::Network,
) -> Element<'a, Message> {
    card::simple(
        Column::new()
            .spacing(10)
            .push(text("Input extended public key:").bold())
            .push(
                Row::new()
                    .push(
                        form::Form::new("Extended public key", form_xpub, |msg| {
                            Message::DefineDescriptor(message::DefineDescriptor::XPubEdited(msg))
                        })
                        .warning(if network == bitcoin::Network::Bitcoin {
                            "Please enter correct xpub"
                        } else {
                            "Please enter correct tpub"
                        })
                        .size(20)
                        .padding(10),
                    )
                    .spacing(10)
                    .push(Container::new(text("/<0;1>/*")).padding(5)),
            )
            .push(
                Row::new()
                    .push(Space::with_width(Length::Fill))
                    .push(if form_xpub.valid {
                        button::primary(None, "Apply").on_press(Message::DefineDescriptor(
                            message::DefineDescriptor::ConfirmXpub,
                        ))
                    } else {
                        button::primary(None, "Apply")
                    }),
            ),
    )
    .width(Length::Units(600))
    .into()
}

fn hw_list_view<'a>(
    i: usize,
    hw: &HardwareWallet,
    chosen: bool,
    processing: bool,
    registered: bool,
) -> Element<'a, Message> {
    let mut bttn = Button::new(
        Row::new()
            .push(
                Column::new()
                    .push(text(format!("{}", hw.kind)).bold())
                    .push(text(format!("fingerprint: {}", hw.fingerprint)).small())
                    .spacing(5)
                    .width(Length::Fill),
            )
            .push_maybe(if chosen && processing {
                Some(
                    Column::new()
                        .push(text("Processing..."))
                        .push(text("Please check your device").small()),
                )
            } else {
                None
            })
            .push_maybe(if registered {
                Some(Column::new().push(icon::circle_check_icon().style(color::SUCCESS)))
            } else {
                None
            })
            .align_items(Alignment::Center)
            .width(Length::Fill),
    )
    .padding(10)
    .style(button::Style::TransparentBorder.into())
    .width(Length::Fill);
    if !processing {
        bttn = bttn.on_press(Message::Select(i));
    }
    Container::new(bttn)
        .width(Length::Fill)
        .style(card::SimpleCardStyle)
        .into()
}

fn layout<'a>(
    progress: (usize, usize),
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Container::new(Scrollable::new(
        Column::new()
            .push(
                Container::new(button::transparent(None, "< Previous").on_press(Message::Previous))
                    .padding(5),
            )
            .push(
                Container::new(text(format!("{}/{}", progress.0, progress.1)))
                    .width(Length::Fill)
                    .center_x(),
            )
            .push(Container::new(content).width(Length::Fill).center_x()),
    ))
    .center_x()
    .height(Length::Fill)
    .width(Length::Fill)
    .style(container::Style::Background)
    .into()
}

mod threshsold_input {
    use crate::ui::{
        component::{button, text::*},
        icon,
    };
    use iced::alignment::{self, Alignment};
    use iced::widget::{Button, Column, Container};
    use iced::{Element, Length};
    use iced_lazy::{self, Component};

    pub struct ThresholdInput<Message> {
        value: usize,
        max: usize,
        on_change: Box<dyn Fn(usize) -> Message>,
    }

    pub fn threshsold_input<Message>(
        value: usize,
        max: usize,
        on_change: impl Fn(usize) -> Message + 'static,
    ) -> ThresholdInput<Message> {
        ThresholdInput::new(value, max, on_change)
    }

    #[derive(Debug, Clone)]
    pub enum Event {
        IncrementPressed,
        DecrementPressed,
    }

    impl<Message> ThresholdInput<Message> {
        pub fn new(
            value: usize,
            max: usize,
            on_change: impl Fn(usize) -> Message + 'static,
        ) -> Self {
            Self {
                value,
                max,
                on_change: Box::new(on_change),
            }
        }
    }

    impl<Message> Component<Message, iced::Renderer> for ThresholdInput<Message> {
        type State = ();
        type Event = Event;

        fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
            match event {
                Event::IncrementPressed => {
                    if self.value < self.max {
                        Some((self.on_change)(self.value.saturating_add(1)))
                    } else {
                        None
                    }
                }
                Event::DecrementPressed => {
                    if self.value > 1 {
                        Some((self.on_change)(self.value.saturating_sub(1)))
                    } else {
                        None
                    }
                }
            }
        }

        fn view(&self, _state: &Self::State) -> Element<Self::Event> {
            let button = |label, on_press| {
                Button::new(label)
                    .style(button::Style::Transparent.into())
                    .width(Length::Units(50))
                    .on_press(on_press)
            };

            Column::new()
                .height(Length::Units(250))
                .width(Length::Units(200))
                .push(button(icon::up_icon().size(50), Event::IncrementPressed))
                .push(text("Threshold:").small().bold())
                .push(
                    Container::new(text(format!("{}/{}", self.value, self.max)).size(50))
                        .height(Length::Fill)
                        .align_y(alignment::Vertical::Center),
                )
                .push(button(icon::down_icon().size(50), Event::DecrementPressed))
                .align_items(Alignment::Center)
                .spacing(10)
                .into()
        }
    }

    impl<'a, Message> From<ThresholdInput<Message>> for Element<'a, Message>
    where
        Message: 'a,
    {
        fn from(numeric_input: ThresholdInput<Message>) -> Self {
            iced_lazy::component(numeric_input)
        }
    }
}
