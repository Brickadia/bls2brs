#![allow(clippy::identity_op)]

use crate::types::{BrickDesc, BrickMapping};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

type RegexHandler = Box<dyn Fn(Captures, &bl_save::Brick) -> Option<BrickMapping> + Sync>;

lazy_static! {
    static ref BLANK_PRINTS: HashSet<&'static str> = vec![
        "Letters/-space",
        "1x2f/blank",
        "2x2f/blank",
    ].into_iter().collect();

    static ref BRICK_ROAD_LANE: BrickDesc = BrickDesc::new("PB_DefaultTile")
        .color_override(brs::Color::from_rgba(51, 51, 51, 255));
    static ref BRICK_ROAD_STRIPE: BrickDesc = BrickDesc::new("PB_DefaultTile")
        .color_override(brs::Color::from_rgba(254, 254, 232, 255));
    static ref GENERIC_DOOR: BrickMapping = vec![
        //frame
        BrickDesc::new("PB_DefaultMicroBrick").size((20, 5, 1)).offset((0, 0, -35)),
        BrickDesc::new("PB_DefaultMicroBrick").size((20, 5, 1)).offset((0, 0, 35)),
        BrickDesc::new("PB_DefaultMicroBrick").size((1, 5, 34)).offset((0, 19, 0)),
        BrickDesc::new("PB_DefaultMicroBrick").size((1, 5, 34)).offset((0, -19, 0)),
        //door
        BrickDesc::new("PB_DefaultMicroBrick").size((18, 1, 34)),
        //handle
        BrickDesc::new("PB_DefaultMicroBrick").size((3, 1, 3)).offset((2, 12, 0)),
        BrickDesc::new("PB_DefaultMicroBrick").size((3, 1, 3)).offset((-2, 12, 0)),
    ];

    pub static ref BRICK_MAP_LITERAL: HashMap<&'static str, BrickMapping> = brick_map_literal![
        // # Correct mappings

        "1x1 Cone" => BrickDesc::new("B_1x1_Cone"),
        "1x1 Round" => BrickDesc::new("B_1x1_Round"),
        "1x1 Octo Plate" => BrickDesc::new("B_1x1F_Octo"),
        "1x1F Round" => BrickDesc::new("B_1x1F_Round"),
        "2x2 Round" => BrickDesc::new("B_2x2_Round"),
        "2x2F Round" => BrickDesc::new("B_2x2F_Round"),
        "Pine Tree" => BrickDesc::new("B_Pine_Tree").offset((0, 0, -6)),
        "2x2 Corner" => BrickDesc::new("B_2x2_Corner").rotation_offset(0),
        "2x2 Octo Plate" => BrickDesc::new("B_2x2F_Octo"),
        "8x8 Grill" => BrickDesc::new("B_8x8_Lattice_Plate"),
        "1x4x2 Picket" => BrickDesc::new("B_Picket_Fence"),
        
        // 1RandomBrickPack 45° to 25° Ramp Adapters
        "45° 25° Adapter A" => BrickDesc::new("PB_DefaultRampInnerCorner").size((15, 10, 6)).rotation_offset(0),
        "45° 25° Adapter B" => BrickDesc::new("PB_DefaultRampInnerCorner").size((10, 15, 6)).rotation_offset(1),
        "45° 25° Adapter C" => BrickDesc::new("PB_DefaultRampCorner").size((15, 10, 6)).rotation_offset(0),
        "45° 25° Adapter D" => BrickDesc::new("PB_DefaultRampCorner").size((10, 15, 6)).rotation_offset(1),
        "-45°-25° Inv Adapter B" => BrickDesc::new("PB_DefaultRampInnerCorner").size((15, 10, 6)).rotation_offset(0).direction_override(brs::Direction::ZNegative),
        "-45°-25° Inv Adapter A" => BrickDesc::new("PB_DefaultRampInnerCorner").size((10, 15, 6)).rotation_offset(1).direction_override(brs::Direction::ZNegative),
        "-45° -25° Inv Adapter D" => BrickDesc::new("PB_DefaultRampCorner").size((15, 10, 6)).rotation_offset(0).direction_override(brs::Direction::ZNegative),
        "-45° -25° Inv Adapter C" => BrickDesc::new("PB_DefaultRampCorner").size((10, 15, 6)).rotation_offset(1).direction_override(brs::Direction::ZNegative),

        // # Approximate mappings

        "2x2 Disc" => BrickDesc::new("B_2x2F_Round"),
        "Music Brick" => BrickDesc::new("PB_DefaultBrick").size((5, 5, 6)),
        "1x4x2 Fence" => BrickDesc::new("PB_DefaultBrick").size((5, 4*5, 2*6)).rotation_offset(0),
        "2x2x1 Octo Cone" => BrickDesc::new("B_2x2_Round"),
        "Gravestone" => BrickDesc::new("B_Gravestone"),
        "House Door" => GENERIC_DOOR.clone(),
        "Plain Door" => GENERIC_DOOR.clone(),

        "2x2x2 Cone" => vec![
            BrickDesc::new("B_2x_Octo_Cone").offset((0, 0, -2)),
            BrickDesc::new("B_1x1F_Round").offset((0, 0, 2*6-2)),
        ],

        "2x2 Octo" => vec![
            BrickDesc::new("B_2x2F_Octo").offset((0, 0, -4)),
            BrickDesc::new("B_2x2F_Octo"),
            BrickDesc::new("B_2x2F_Octo").offset((0, 0, 4)),
        ],

        "Castle Wall" => vec![
            BrickDesc::new("PB_DefaultTile").size((5, 5, 6*6)).offset((0, -10, 0)),
            BrickDesc::new("PB_DefaultTile").size((5, 5, 6*6)).offset((0, 10, 0)),
            BrickDesc::new("PB_DefaultTile").size((5, 5, 3*6)).offset((0, 0, -9*2)),
            BrickDesc::new("PB_DefaultTile").size((5, 5, 4*2)).offset((0, 0, 14*2)),
        ],

        "1x4x5 Window" => vec![
            BrickDesc::new("PB_DefaultBrick").size((5, 4*5, 2)).rotation_offset(0).offset((0, 0, -14*2)),
            BrickDesc::new("PB_DefaultTile").size((5, 4*5, 5*6-2)).rotation_offset(0).offset((0, 0, 2))
                .color_override(brs::Color::from_rgba(255, 255, 255, 76)),
        ],

        "1x4x2 Bars" => vec![
            BrickDesc::new("PB_DefaultMicroBrick").size((5, 20, 2)).offset((0, 0, -10)),
            BrickDesc::new("PB_DefaultMicroBrick").size((5, 20, 1)).offset((0, 0, 11)),
            BrickDesc::new("PB_DefaultPole").size((3, 3, 1)).offset((-15, 0, -7)),
            BrickDesc::new("PB_DefaultPole").size((3, 3, 1)).offset((-5, 0, -7)),
            BrickDesc::new("PB_DefaultPole").size((3, 3, 1)).offset((5, 0, -7)),
            BrickDesc::new("PB_DefaultPole").size((3, 3, 1)).offset((15, 0, -7)),
            BrickDesc::new("PB_DefaultPole").size((2, 2, 8)).offset((15, 0, 2)),
            BrickDesc::new("PB_DefaultPole").size((2, 2, 8)).offset((5, 0, 2)),
            BrickDesc::new("PB_DefaultPole").size((2, 2, 8)).offset((-5, 0, 2)),
            BrickDesc::new("PB_DefaultPole").size((2, 2, 8)).offset((-15, 0, 2)),
        ],

        "Treasure Chest" => vec![
            // Body
            BrickDesc::new("PB_DefaultMicroBrick").size((20, 10, 2)).offset((0, 0, -8)),
            BrickDesc::new("PB_DefaultMicroBrick").size((2, 2, 2)).offset((8, 18, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((2, 2, 2)).offset((8, -18, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((2, 2, 2)).offset((-8, 18, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((2, 2, 2)).offset((-8, -18, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((4, 2, 2)).offset((8, 0, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((4, 2, 2)).offset((-8, 0, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((19, 6, 2)).offset((0, 0, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((6, 1, 2)).offset((8, 10, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((6, 1, 2)).offset((8, -10, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((6, 1, 2)).offset((-8, 10, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((6, 1, 2)).offset((-8, -10, -4)),
            BrickDesc::new("PB_DefaultMicroBrick").size((20, 10, 4)).offset((0, 0, 2)),
            BrickDesc::new("PB_DefaultMicroBrick").size((20, 6, 2)).offset((0, 0, 8)),
            BrickDesc::new("PB_DefaultMicroWedge").size((2, 20, 2)).offset((-8, 0, 8)).microwedge_rotate(true).rotation_offset(2),
            BrickDesc::new("PB_DefaultMicroWedge").size((2, 20, 2)).offset((8, 0, 8)).microwedge_rotate(true).rotation_offset(0),
            // Lock
            BrickDesc::new("PB_DefaultMicroBrick").size((4, 1, 2)).offset((-11, 0, 2)).non_priority(true)
                .color_override(brs::Color::from_rgba(255, 255, 0, 255)),
            BrickDesc::new("PB_DefaultMicroBrick").size((2, 1, 1)).offset((-11, 0, -1)).non_priority(true)
                .color_override(brs::Color::from_rgba(255, 255, 0, 255)),
            BrickDesc::new("PB_DefaultMicroWedge").size((1, 1, 1)).offset((-11, 3, -1)).non_priority(true).microwedge_rotate(true)
                .color_override(brs::Color::from_rgba(255, 255, 0, 255)).rotation_offset(3)
                .direction_override(brs::Direction::ZNegative),
            BrickDesc::new("PB_DefaultMicroWedge").size((1, 1, 1)).offset((-11, -3, -1)).non_priority(true).microwedge_rotate(true)
                .color_override(brs::Color::from_rgba(255, 255, 0, 255)).rotation_offset(1)
                .direction_override(brs::Direction::ZNegative),
        ],

        "32x32 Road" => vec![
            // left and right sidewalks
            BrickDesc::new("PB_DefaultBrick").size((9*5, 32*5, 2)).offset((0, -115, 0)),
            BrickDesc::new("PB_DefaultBrick").size((9*5, 32*5, 2)).offset((0, 115, 0)),
            // left and right stripes
            BRICK_ROAD_STRIPE.clone().size((1*5, 32*5, 2)).offset((0, -65, 0)),
            BRICK_ROAD_STRIPE.clone().size((1*5, 32*5, 2)).offset((0, 65, 0)),
            // lanes
            BRICK_ROAD_LANE.clone().size((6*5, 32*5, 2)).offset((0, -6*5, 0)),
            BRICK_ROAD_LANE.clone().size((6*5, 32*5, 2)).offset((0, 6*5, 0)),
        ],

        // Orientations are relative to this camera position on Beta City:
        // 39.5712 0.0598862 14.5026 0.999998 -0.0007625 0.00180403 0.799784
        "32x32 Road T" => vec![
            BrickDesc::new("PB_DefaultBrick").size((9*5, 32*5, 2)).offset((0, -115, 0)), // top
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((-115, 115, 0)), // bottom left
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((115, 115, 0)), // bottom right
            BRICK_ROAD_STRIPE.clone().size((1*5, 32*5, 2)).offset((0, -65, 0)), // straight top
            BRICK_ROAD_STRIPE.clone().size((1*5, 32*5, 2)).offset((0, 65, 0)), // straight bottom
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((-13*5, 23*5, 0)), // bottom left
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((13*5, 23*5, 0)), // bottom right
            BRICK_ROAD_LANE.clone().size((6*5, 32*5, 2)).offset((0, -6*5, 0)), // straight top
            BRICK_ROAD_LANE.clone().size((6*5, 32*5, 2)).offset((0, 6*5, 0)), // straight bottom
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).rotation_offset(0).offset((-6*5, 23*5, 0)), // bottom left
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).rotation_offset(0).offset((6*5, 23*5, 0)), // bottom right
        ],

        // Orientations are relative to this camera position on Beta City:
        // -56.5 -35 4 0 0 1 3.14159
        "32x32 Road X" => vec![
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((-23*5, -23*5, 0)), // top left
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((23*5, -23*5, 0)), // top right
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((-23*5, 23*5, 0)), // bottom left
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((23*5, 23*5, 0)), // bottom right
            BRICK_ROAD_STRIPE.clone().size((1*5, 1*5, 2)).offset((13*5, -13*5, 0)), // corner top left
            BRICK_ROAD_STRIPE.clone().size((1*5, 1*5, 2)).offset((13*5, 13*5, 0)), // corner right right
            BRICK_ROAD_STRIPE.clone().size((1*5, 1*5, 2)).offset((-13*5, -13*5, 0)), // corner bottom left
            BRICK_ROAD_STRIPE.clone().size((1*5, 1*5, 2)).offset((-13*5, 13*5, 0)), // corner bottom right
            BRICK_ROAD_STRIPE.clone().size((1*5, 12*5, 2)).rotation_offset(0).offset((-13*5, 0, 0)), // inner bottom
            BRICK_ROAD_STRIPE.clone().size((1*5, 12*5, 2)).rotation_offset(0).offset((13*5, 0, 0)), // inner top
            BRICK_ROAD_STRIPE.clone().size((1*5, 12*5, 2)).offset((0, -13*5, 0)), // inner left
            BRICK_ROAD_STRIPE.clone().size((1*5, 12*5, 2)).offset((0, 13*5, 0)), // inner right
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((-13*5, 23*5, 0)), // right bottom
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((13*5, 23*5, 0)), // right top
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((-13*5, -23*5, 0)), // left bottom
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((13*5, -23*5, 0)), // left top
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).offset((-23*5, -13*5, 0)), // bottom left
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).offset((-23*5, 13*5, 0)), // bottom right
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).offset((23*5, -13*5, 0)), // top left
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).offset((23*5, 13*5, 0)), // top right
            BRICK_ROAD_LANE.clone().size((6*5, 6*5, 2)).offset((-6*5, -6*5, 0)), // inner bottom left
            BRICK_ROAD_LANE.clone().size((6*5, 6*5, 2)).offset((-6*5, 6*5, 0)), // inner bottom right
            BRICK_ROAD_LANE.clone().size((6*5, 6*5, 2)).offset((6*5, -6*5, 0)), // inner top left
            BRICK_ROAD_LANE.clone().size((6*5, 6*5, 2)).offset((6*5, 6*5, 0)), // inner top right
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).rotation_offset(0).offset((-6*5, 23*5, 0)), // right bottom
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).rotation_offset(0).offset((6*5, 23*5, 0)), // right top
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).rotation_offset(0).offset((-6*5, -23*5, 0)), // left bottom
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).rotation_offset(0).offset((6*5, -23*5, 0)), // left top
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).offset((-23*5, -6*5, 0)), // bottom left
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).offset((-23*5, 6*5, 0)), // bottom right
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).offset((23*5, -6*5, 0)), // top left
            BRICK_ROAD_LANE.clone().size((6*5, 9*5, 2)).offset((23*5, 6*5, 0)), // top right
        ],

        // Orientations are relative to this camera position on Beta City:
        // -25.9168 -110.523 12.5993 0.996034 0.0289472 -0.0841301 0.665224
        "32x32 Road C" => vec![
            // sidewalks
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((-115, 115, 0)), // top left
            BrickDesc::new("PB_DefaultBrick").size((9*5, 9*5, 2)).offset((115, -115, 0)), // bottom right
            BrickDesc::new("PB_DefaultBrick").size((9*5, 23*5, 2)).rotation_offset(0).offset((115, 45, 0)), // bottom left
            BrickDesc::new("PB_DefaultBrick").size((9*5, 23*5, 2)).offset((-45, -115, 0)), // top right
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).offset((-115, 65, 0)), // inner right
            BRICK_ROAD_STRIPE.clone().size((1*5, 9*5, 2)).rotation_offset(0).offset((-65, 115, 0)), // inner bottom
            BRICK_ROAD_STRIPE.clone().size((1*5, 22*5, 2)).offset((-50, -65, 0)), // top right
            BRICK_ROAD_STRIPE.clone().size((1*5, 22*5, 2)).rotation_offset(0).offset((65, 50, 0)), // bottom left
            BRICK_ROAD_STRIPE.clone().size((1*5, 1*5, 2)).offset((65, -65, 0)), // bottom right
            BRICK_ROAD_STRIPE.clone().size((1*5, 1*5, 2)).rotation_offset(0).offset((-65, 65, 0)), // inner bottom right
            BRICK_ROAD_LANE.clone().size((6*5, 10*5, 2)).offset((-22*5, 6*5, 0)), // top left
            BRICK_ROAD_LANE.clone().size((6*5, 16*5, 2)).offset((-16*5, -6*5, 0)), // top right
            BRICK_ROAD_LANE.clone().size((6*5, 16*5, 2)).rotation_offset(0).offset((6*5, 16*5, 0)), // bottom left
            BRICK_ROAD_LANE.clone().size((6*5, 10*5, 2)).rotation_offset(0).offset((-6*5, 22*5, 0)), // left top
            BRICK_ROAD_LANE.clone().size((6*5, 6*5, 2)).offset((-6*5, 6*5, 0)), // inner top left
            BRICK_ROAD_LANE.clone().size((6*5, 6*5, 2)).offset((6*5, -6*5, 0)), // inner bottom right
        ],
    ];

    pub static ref BRICK_MAP_REGEX: Vec<(Regex, RegexHandler)> = brick_map_regex![
        // TODO: Consider trying to handle fractional sizes that sometimes occur
        // TODO: Remove (?: Print)? when prints exist
        r"^(\d+)x(\d+)(?:x(\d+)|([Ff])|([Hh]))?( Print)?$" => |captures, from| {
            let width: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
            let length: u32 = captures.get(2).unwrap().as_str().parse().ok()?;
            let z: u32 = if captures.get(4).is_some() { // F
                2
            } else if captures.get(5).is_some() { // H
                4
            } else { // x(Z)
                captures
                    .get(3)
                    .map(|g| g.as_str().parse::<u32>().ok())
                    .unwrap_or(Some(1))?
                    * 6
            };

            let print = captures.get(6).is_some();
            let asset = if print && BLANK_PRINTS.contains(from.base.print.as_str()) {
                "PB_DefaultTile"
            } else {
                "PB_DefaultBrick"
            };
            let rotation_offset = if print { 0 } else { 1 };

            Some(vec![BrickDesc::new(asset)
                .size((width * 5, length * 5, z))
                .rotation_offset(rotation_offset)])
        },

        // TODO: Remove (?: Print)? when prints exist
        r"^(-)?(25|45|65|72|80)° (Inv )?Ramp(?: (\d+)x)?( Corner)?(?: Print)?$" => |captures, _| {
            let neg = captures.get(1).is_some();
            let inv = captures.get(3).is_some();
            let corner = captures.get(5).is_some();

            if inv && !corner {
                return None;
            }

            let asset = if neg {
                if inv {
                    "PB_DefaultRampInnerCornerInverted"
                } else if corner {
                    "PB_DefaultRampCornerInverted"
                } else {
                    "PB_DefaultRampInverted"
                }
            } else if inv {
                "PB_DefaultRampInnerCorner"
            } else if corner {
                "PB_DefaultRampCorner"
            } else {
                "PB_DefaultRamp"
            };

            let degree_str = captures.get(2).unwrap().as_str();

            let (x, z) = if degree_str == "25" {
                (15, 6)
            } else if degree_str == "45" {
                (10, 6)
            } else if degree_str == "65" {
                (10, 12)
            } else if degree_str == "72" {
                (10, 18)
            } else if degree_str == "80" {
                (10, 30)
            } else {
                return None;
            };

            let mut y = x;

            if let Some(group) = captures.get(4) {
                if corner {
                    return None;
                }

                let length: u32 = group.as_str().parse().ok()?;
                y = length * 5;
            }

            Some(vec![BrickDesc::new(asset).size((x, y, z)).rotation_offset(0)])
        },

        r"(?P<angle>25|45)° Crest (?:(?P<end>End)|(?P<corner>Corner)|(?P<length>\d+)x)" => |captures, _| {
            let (z, offset) = match captures.name("angle").unwrap().as_str() {
                s if s == "25" => (4, -2),
                s if s == "45" => (6, 0),
                _ => return None,
            };

            let (asset, x, y, rotation) = if captures.name("end").is_some() {
                ("PB_DefaultRampCrestEnd", 10, 5, 2)
            } else if captures.name("corner").is_some() {
                ("PB_DefaultRampCrestCorner", 10, 10, 0)
            } else {
                let length: u32 = captures.name("length").unwrap().as_str().parse().ok()?;
                ("PB_DefaultRampCrest", 10, length * 5, 0)
            };

            Some(vec![BrickDesc::new(asset)
                .size((x, y, z))
                .rotation_offset(rotation)
                .offset((0, 0, offset))])
        },

        r"^(\d+)x(\d+)F Tile$" => |captures, _| {
            let width: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
            let length: u32 = captures.get(2).unwrap().as_str().parse().ok()?;
            Some(vec![BrickDesc::new("PB_DefaultTile").size((width * 5, length * 5, 2))])
        },
        r"^(\d+)x(\d+) Base$" => |captures, _| {
            let width: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
            let length: u32 = captures.get(2).unwrap().as_str().parse().ok()?;
            Some(vec![BrickDesc::new("PB_DefaultBrick").size((width * 5, length * 5, 2))])
        },
        r"^(\d+)x Cube$" => |captures, _| {
            let size: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
            Some(vec![BrickDesc::new("PB_DefaultBrick").size((size * 5, size * 5, size * 5))])
        },
        r"^(?P<size>\d+)x (?:(?P<cube>Cube)|(?P<ramp>Ramp)|(?P<cornera>CornerA|CorA)|(?P<cornerb>CornerB|CorB)|(?P<cornerc>CornerC|CorC)|(?P<cornerd>CornerD|CorD)|(?P<wedge>Wedge))(?:(?P<steep> Steep)|(?P<three_quarters> 3/4h)|(?P<half> 1/2h)|(?P<quarter> 1/4h)| )?(?P<inv> Inv.)?$" => |captures, _| {
            let size: u32 = captures.name("size").unwrap().as_str().parse().ok()?;
            let height = if captures.name("steep").is_some() {
                size * 2 * 5
            } else if captures.name("three_quarters").is_some() {
                match size {
                    8 => 5 * 6,
                    _ => return None
                }
            } else if captures.name("half").is_some() {
                size / 2 * 5
            } else if captures.name("quarter").is_some() {
                size / 4 * 5
            } else {
                size * 5
            };
            let (asset, mut rotation, use_offset, mw) = if captures.name("cube").is_some() {
                ("PB_DefaultMicroBrick", 1, false, false)
            } else if captures.name("wedge").is_some() {
                ("PB_DefaultMicroWedge", 2, false, false)
            } else if captures.name("ramp").is_some() {
                ("PB_DefaultMicroWedge", 3, false, true)
            } else if captures.name("cornera").is_some() {
                ("PB_DefaultMicroWedgeTriangleCorner", 2, false, false)
            } else if captures.name("cornerb").is_some() {
                ("PB_DefaultMicroWedgeOuterCorner", 2, false, false)
            } else if captures.name("cornerc").is_some() {
                ("PB_DefaultMicroWedgeCorner", 2, false, false)
            } else if captures.name("cornerd").is_some() {
                ("PB_DefaultMicroWedgeInnerCorner", 2, false, false)
            } else {
                unreachable!()
            };
            let offset = if use_offset {
                (0, (size * 10) as i32, 0)
            } else {
                (0, 0, 0)
            };
            let (direction, imr) = if captures.name("inv").is_some() {
                if captures.name("ramp").is_some() {
                    rotation += 2;
                    (brs::Direction::ZNegative, false)
                } else {
                    rotation += 3;
                    (brs::Direction::ZNegative, true)
                }
            } else {
                (brs::Direction::ZPositive, false)
            };

            Some(vec![BrickDesc::new(asset)
                .size((size * 5, size * 5, height))
                .offset(offset)
                .rotation_offset(rotation)
                .microwedge_rotate(mw)
                .inverted_modter_rotate(imr)
                .direction_override(direction)])
        },
        r"(\d+)x(\d+)x?(?P<height>\d+)? Arch(?P<up> Up)?" => |captures, _| {
            let width: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
            let length: u32 = captures.get(2).unwrap().as_str().parse().ok()?;
            let height: u32 = if captures.name("height").is_some() {
                captures.name("height").unwrap().as_str().parse().ok()?
            } else {
                match length {
                    5 => 2,
                    8 => 3,
                    _ => 1
                }
            };
            let direction = if captures.name("up").is_some() {
                brs::Direction::ZNegative
            } else {
                brs::Direction::ZPositive
            };
            Some(vec![BrickDesc::new("PB_DefaultArch")
                .size((width * 5, length * 5, height * 6))
                .direction_override(direction)
            ])
        },
        // 1RandomPack Panels
        r"^(\d)h Panel (?P<corner>Corner )?(?P<length>\d)x" => |captures, _| {
            let height: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
            let length: u32 = captures.name("length").unwrap().as_str().parse().ok()?;

            if captures.name("corner").is_some() {
                Some(vec![
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 5, 2)).offset((0, 0, 2 - (height*6) as i32)),
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 1, 4)).offset((-4, 0, 2)),
                    BrickDesc::new("PB_DefaultMicroBrick").size((1, 4, 4)).offset((1, -4, 2)),
                ])
            } else if height == 1 {
                Some(vec![
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 5, 2)).offset((0, 0, 2 - (height*6) as i32)),
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 1, 4)).offset((-4, 0, 2))
                ])
            } else {
                Some(vec![
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 5, 2)).offset((0, 0, 2 - (height*6) as i32)),
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 1, height*6 - 4)).offset((-4, 0, 0)),
                    BrickDesc::new("PB_DefaultMicroBrick").size((length*5, 5, 2)).offset((0, 0, (height*6) as i32 - 2))
                ])
            }
        },
        // 1RandomPack Center Ramps
        r"^(-)?(\d+)° Center (Diag )?Ramp 1x" => |captures, _| {
            let neg = captures.get(1).is_some();
            let angle = captures.get(2).unwrap().as_str();
            let diag = captures.get(3).is_some();
            let (z, x) = match angle {
                "18" => (6, 15),
                "25" => (6, 10),
                "45" => (6, 5),
                "65" => (12, 5),
                "72" => (18, 5),
                "80" => (30, 5),
                _ => return None
            };
            let (dir, iwr) = if neg {
                (brs::Direction::ZNegative, true)
            } else {
                (brs::Direction::ZPositive, false)
            };
            let (dir2, iwr2) = if diag {
                (brs::Direction::ZNegative, true)
            } else {
                (dir, iwr)
            };
            Some(vec![
                BrickDesc::new("PB_DefaultBrick").size((5, 5, z)).direction_override(dir),
                BrickDesc::new("PB_DefaultWedge").size((x, 5, z)).offset((-(x as i32 + 5), 0, 0))
                    .rotation_offset(2).direction_override(dir).inverted_wedge_rotate(iwr),
                BrickDesc::new("PB_DefaultWedge").size((x, 5, z)).offset((x as i32 + 5, 0, 0))
                    .rotation_offset(0).direction_override(dir2).inverted_wedge_rotate(iwr2),
            ])
        },
    ];
}
