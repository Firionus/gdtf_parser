//!Holds the DMXChannel and it's children
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
use crate::utils::deparse;
use crate::utils::deparse::{DeparseSingle, DeparseVec};
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::dmx_break::DMXBreak;
use crate::utils::units::highlight::Highlight;
use crate::utils::units::name::Name;
use crate::utils::units::node::node_dmx_channel_initial_function::NodeDmxChannelInitialFunction;
use crate::utils::units::offset::Offset;

pub mod logical_channel;

///This section defines the DMX channe
#[derive(Debug)]
pub struct DMXChannel {
    ///Number of the DMXBreak; Default value: 1; Special value: “Overwrite” – means that this number will be overwritten by Geometry Reference; Size: 4 bytes
    pub dmx_break: DMXBreak,
    ///Relative addresses of the current DMX channel from highest to least significant
    pub offset: Offset,
    ///Link to the channel function that will be activated by default for this DMXChannel;
    pub initial_function: NodeDmxChannelInitialFunction,
    ///Highlight value for current channel; Special value: “None”. Default value: “None”.
    pub highlight: Highlight,
    ///Name of the geometry the current channel controls.
    pub geometry: Name,
    ///List of logical channels
    pub logical_channels: Vec<LogicalChannel>,
}

impl DeparseSingle for DMXChannel {
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut dmx_break = DMXBreak::default();
        let mut offset = Offset::default();
        let mut initial_function: NodeDmxChannelInitialFunction = Default::default();
        let mut highlight = Highlight::default();
        let mut geometry = Default::default();
        let mut logical_channels: Vec<LogicalChannel> = Vec::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"DMXBreak" => dmx_break = deparse::attr_to_str(&attr).into(),
                b"Offset" => offset = deparse::attr_to_str(&attr).into(),
                b"InitialFunction" => initial_function = attr.into(),
                b"Highlight" => highlight = deparse::attr_to_str(&attr).into(),
                b"Geometry" => geometry = attr.into(),
                _ => {}
            }
        }


        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"LogicalChannel" {
                        logical_channels.push(LogicalChannel::single_from_event(reader, e)?);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Eof => {
                    break;
                }

                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
        buf.clear();

        Ok(Self {
            dmx_break,
            offset,
            initial_function,
            highlight,
            geometry,
            logical_channels,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXChannel"
    }

    fn single_event_name() -> String {
        "DMXChannel".to_string()
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for DMXChannel {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        if self.dmx_break != other.dmx_break {
            return Self::print_structs_not_equal(&self.dmx_break, &other.dmx_break, log);
        }
        if self.offset != other.offset {
            return Self::print_structs_not_equal(&self.offset, &other.offset, log);
        }
        if self.highlight != other.highlight {
            return Self::print_structs_not_equal(&self.highlight, &other.highlight, log);
        }
        self.initial_function.is_eq_allow_empty(&other.initial_function, log) &&
            self.geometry.is_eq_allow_empty(&other.geometry, log) &&
            LogicalChannel::is_vec_eq_unordered(&self.logical_channels, &other.logical_channels, log)
    }
}

#[cfg(test)]
impl TestDeparseSingle for DMXChannel {
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.is_eq_allow_empty(compare, false)
    }
}

impl DeparseVec for DMXChannel {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXChannels"
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::dmx_channel::DMXChannel;
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::dmx_break::DMXBreak;
    use crate::utils::units::dmx_value::DMXValue;
    use crate::utils::units::highlight::Highlight;
    use crate::utils::units::master::Master;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_dmx_channel_initial_function::NodeDmxChannelInitialFunction;
    use crate::utils::units::node::node_logical_channel_attribute::NodeLogicalChannelAttribute;
    use crate::utils::units::offset::Offset;
    use crate::utils::units::snap::Snap;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        DMXChannel {
            dmx_break: DMXBreak::Value(1),
            offset: Offset::Value(vec![1]),
            initial_function: NodeDmxChannelInitialFunction::new_from_strs_unchecked(vec!["Beam_Shutter1", "Shutter1", "Open"]),
            highlight: Highlight::Value(DMXValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: vec![],
                }
            ],
        }.test(
            r#"
            <DMXChannel DMXBreak="1" Geometry="Beam" Highlight="8/1" InitialFunction="Beam_Shutter1.Shutter1.Open" Offset="1">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
            </DMXChannel>
            "#
        );
        Ok(())
    }

    #[test]
    fn test_normal_2() -> Result<(), GdtfError> {
        DMXChannel {
            dmx_break: DMXBreak::Value(2),
            offset: Offset::Value(vec![1, 2]),
            initial_function: NodeDmxChannelInitialFunction::new_from_strs_unchecked(vec!["Beam_Shutter1", "Shutter1", "Open"]),
            highlight: Highlight::Value(DMXValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: vec![],
                }
            ],
        }.test(
            r#"
            <DMXChannel DMXBreak="2" Geometry="Beam" Highlight="8/1" InitialFunction="Beam_Shutter1.Shutter1.Open" Offset="1,2">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
            </DMXChannel>
            "#
        );
        Ok(())
    }

    #[test]
    fn test_normal_3() -> Result<(), GdtfError> {
        DMXChannel {
            dmx_break: DMXBreak::Overwrite,
            offset: Offset::Value(vec![1, 2]),
            initial_function: NodeDmxChannelInitialFunction::new_from_strs_unchecked(vec!["Beam_Shutter1", "Shutter1", "Open"]),
            highlight: Highlight::Value(DMXValue {
                initial_value: 8,
                n: 1,
                is_byte_shifting: false,
            }),
            geometry: Name::new("Beam")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: vec![],
                }
            ],
        }.test(
            r#"
            <DMXChannel DMXBreak="Overwrite" Geometry="Beam" Highlight="8/1" InitialFunction="Beam_Shutter1.Shutter1.Open" Offset="1,2">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
            </DMXChannel>
            "#
        );
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), GdtfError> {
        DMXChannel {
            dmx_break: DMXBreak::Value(1),
            offset: Offset::None,
            initial_function: NodeDmxChannelInitialFunction::none(),
            highlight: Highlight::None,
            geometry: Name::new("")?,
            logical_channels: vec![
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::No,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: vec![],
                },
                LogicalChannel {
                    attribute: NodeLogicalChannelAttribute::new_from_attribute_names(vec![AttributeName::Shutter_n_(1)])?,
                    snap: Snap::Yes,
                    master: Master::None,
                    mib_fade: 0.0,
                    dmx_change_time_limit: 0.0,
                    channel_functions: vec![],
                }
            ],
        }.test(
            r#"
            <DMXChannel DMXBreak="" Geometry="" Highlight="" InitialFunction="" Offset="">
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No"></LogicalChannel>
                <LogicalChannel Attribute="Shutter1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="Yes"></LogicalChannel>
            </DMXChannel>
            "#
        );
        Ok(())
    }

    #[test]
    fn test_faulty() -> Result<(), GdtfError> {
        DMXChannel {
            dmx_break: DMXBreak::Value(1),
            offset: Offset::None,
            initial_function: NodeDmxChannelInitialFunction::none(),
            highlight: Highlight::None,
            geometry: Name::new("")?,
            logical_channels: vec![],
        }.test(
            r#"
            <DMXChannel>
            </DMXChannel>
            "#
        );
        Ok(())
    }
}