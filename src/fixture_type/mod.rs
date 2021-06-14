//! Holds the GDTF FixtureType and it's children
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::AttributeDefinitions;
use crate::fixture_type::dmx_mode::DmxMode;
use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::errors::GdtfError::QuickXmlError;
use crate::utils::read;
use crate::utils::read::{GdtfReadError, ReadGdtf};
use crate::utils::units::guid::Guid;
use crate::utils::units::name::Name;

pub mod attribute_definitions;
pub mod dmx_mode;

///The FixtureType node_2 is the starting point of the description of the fixture type
#[derive(Debug, PartialEq, Clone)]
pub struct FixtureType {
    ///Name of the fixture type.
    pub name: Name,
    /// Shortened name of the fixture type.
    pub short_name: String,
    ///Detailed name of the fixture type.
    pub long_name: String,
    ///Manufacturer of the fixture type.
    pub manufacturer: String,
    /// Description of the fixture type.
    pub description: String,
    ///Unique number of the fixture type.
    pub fixture_type_id: Guid,
    /// File name without extension containing description of the thumbnail.Use the following as a resource file:
    pub thumbnail: Option<String>,
    ///GUID of the referenced fixture type
    pub ref_ft: Option<Guid>,
    ///This section defines all attributes that are used in the fixture type.
    pub attribute_definitions: AttributeDefinitions,
    //Defines the physical or virtual color wheels, gobo wheels, media server content and others.
    // pub wheels: Option<Wheels>,
    //Contains additional physical descriptions.
    // pub physical_descriptions: Option<PhysicalDescriptions>,
    //Contains models of physically separated parts of the device.
    // pub models: Option<Models>,
    //Describes physically separated parts of the device.
    //pub geometries: Geometries,
    ///Contains descriptions of the DMX modes.
    pub dmx_modes: HashMap<Name, DmxMode>,
    //Describe the history of the fixture type.
    //pub revisions: Option<Revisions>,
    //Is used to transfer user - defined and fixture type specific presets to other show files.
    //pub ft_presets: Option<FTPresets>,
    //Specifies supported protocols.
    //pub protocols: Option<Protocols>,
}

impl DeparseSingle for FixtureType {
    type PrimaryKey = ();
    type Error = GdtfError;

    const NODE_NAME_DS: &'static [u8] = b"FixtureType";

    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(Option<Self::PrimaryKey>, Self), GdtfError> where
        Self: Sized {
        let mut name = Name::default();
        let mut short_name = String::new();
        let mut long_name = String::new();
        let mut manufacturer = String::new();
        let mut description = String::new();
        let mut fixture_type_id = Guid::dummy();
        let mut thumbnail = None;
        let mut ref_ft: Option<Guid> = None;

        for attr in event.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Name::new_from_attr(attr)?,
                b"ShortName" => short_name = read::attr_to_string(attr),
                b"LongName" => long_name = read::attr_to_string(attr),
                b"Manufacturer" => manufacturer = read::attr_to_string(attr),
                b"Description" => description = read::attr_to_string(attr),
                b"FixtureTypeID" => fixture_type_id = Guid::new_from_attr(attr)?,
                b"Thumbnail" => thumbnail = read::attr_to_string_option(attr),
                b"RefFT" => ref_ft = match Guid::new_from_attr(attr) {
                    Ok(guid) => Some(guid),
                    Err(_) => None
                },

                _ => {}
            }
        }


        let mut attribute_definitions: Option<AttributeDefinitions> = None;
        let mut dmx_modes: Option<HashMap<Name, DmxMode>> = None;
        if has_children {
            let mut buf: Vec<u8> = Vec::new();
            let mut tree_down = 0;
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) => {
                        match e.name() {
                            AttributeDefinitions::NODE_NAME => attribute_definitions = Some(AttributeDefinitions::read_single_from_event(reader, e, true)?.1),
                            b"DMXModes" => {
                                dmx_modes = Some(DmxMode::read_hash_map_from_event(reader, e, true)?);
                            }
                            _ => tree_down += 1
                        }
                    }
                    Ok(Event::Empty(e)) => {
                        match e.name() {
                            AttributeDefinitions::NODE_NAME => attribute_definitions = Some(AttributeDefinitions::read_single_from_event(reader, e, false)?.1),
                            b"DMXModes" => {
                                dmx_modes = Some(DmxMode::read_hash_map_from_event(reader, e, false)?);
                            }
                            _ => tree_down += 1
                        }
                    }
                    Ok(Event::End(_)) => {
                        tree_down -= 1;
                        if tree_down <= 0 { break; }
                    }
                    Ok(Event::Eof) => {
                        break;
                    }
                    Err(e) => return Err(QuickXmlError(e)),
                    _ => {}
                }
            }
            buf.clear();
        }

        if attribute_definitions.is_none() {
            return Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME_DS, AttributeDefinitions::NODE_NAME).into());
        }
        let attribute_definitions = attribute_definitions.unwrap();
        if dmx_modes.is_none() {
            return Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME_DS, DmxMode::NODE_NAME).into());
        }
        let dmx_modes = dmx_modes.unwrap();

        Ok((None, Self {
            name,
            short_name,
            long_name,
            manufacturer,
            description,
            fixture_type_id,
            thumbnail,
            ref_ft,
            dmx_modes,
            attribute_definitions,
        }))
    }
}
