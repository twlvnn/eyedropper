use std::str::FromStr;

use gtk::{
    gio,
    prelude::{SettingsExt, WidgetExt},
};

use crate::{config, widgets::color_format_row::ColorFormatRow};

use super::{
    color::{Color, ColorError},
    color_names,
    illuminant::Illuminant,
    parser,
    position::AlphaPosition,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "Notation")]
pub enum Notation {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Hsv,
    Cmyk,
    Xyz,
    Lab,
    Hwb,
    Hcl,
    Name,
    Lms,
    HunterLab,
    Oklab,
    Oklch,
}

impl Notation {
    pub fn parse(&self, input: &str) -> Result<Color, ColorError> {
        let settings = gio::Settings::new(config::APP_ID);
        let ten_deg_observer = settings.int("cie-standard-observer") == 1;
        let illuminant = Illuminant::from(settings.int("cie-illuminants") as u32);
        let (_, color) = match self {
            Notation::Hex => parser::hex_color(
                input,
                AlphaPosition::from(settings.int("alpha-position") as u32),
            ),
            Notation::Rgb => parser::rgb(input),
            Notation::Hsl => parser::hsl(input),
            Notation::Hsv => parser::hsv(input),
            Notation::Cmyk => parser::cmyk(input),
            Notation::Xyz => parser::xyz(input),
            Notation::Lab => parser::cielab(input, illuminant, ten_deg_observer),
            Notation::Hwb => parser::hwb(input),
            Notation::Hcl => parser::lch(input),
            Notation::Lms => parser::lms(input),
            Notation::HunterLab => parser::hunter_lab(input, illuminant, ten_deg_observer),
            Notation::Oklab => parser::oklab(input),
            Notation::Oklch => parser::oklch(input),
            Notation::Name => {
                return color_names::color(input, true, true, true, true)
                    .ok_or(ColorError::ParsingError("No name found".to_owned()));
            }
        }?;
        Ok(color)
    }

    pub fn as_str(&self, color: Color) -> String {
        let settings = gio::Settings::new(config::APP_ID);
        let formatter = super::formatter::ColorFormatter::with_alpha_position(
            color,
            AlphaPosition::from(settings.int("alpha-position") as u32),
        );
        match self {
            Notation::Hex => formatter.hex_code(),
            Notation::Rgb => formatter.rgb(),
            Notation::Hsl => formatter.hsl(),
            Notation::Hsv => formatter.hsv(),
            Notation::Cmyk => formatter.cmyk(),
            Notation::Xyz => formatter.xyz(),
            Notation::Lab => formatter.cie_lab(),
            Notation::Hwb => formatter.hwb(),
            Notation::Hcl => formatter.hcl(),
            Notation::Lms => formatter.lms(),
            Notation::HunterLab => formatter.hunter_lab(),
            Notation::Oklab => formatter.oklab(),
            Notation::Oklch => formatter.oklch(),
            Notation::Name => color_names::name(color, true, true, true, true)
                .unwrap_or(gettextrs::gettext("Not named")),
        }
    }

    pub fn widget(&self) -> ColorFormatRow {
        ColorFormatRow::new(self)
    }

    pub fn display_copy_string(&self) -> String {
        gettextrs::gettext(match self {
            Notation::Hex => "Copy Hex Code",
            Notation::Rgb => "Copy RGB",
            Notation::Hsl => "Copy HSL",
            Notation::Hsv => "Copy HSV",
            Notation::Cmyk => "Copy CMYK",
            Notation::Xyz => "Copy Xyz",
            Notation::Lab => "Copy CIELAB",
            Notation::Hwb => "Copy HWB",
            Notation::Hcl => "Copy CIELCh / HCL",
            Notation::Lms => "Copy LMS",
            Notation::HunterLab => "Copy Hunter Lab",
            Notation::Oklab => "Copy Oklab",
            Notation::Oklch => "Copy Oklch",
            Notation::Name => "Copy Name",
        })
    }
}

impl FromStr for Notation {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().trim() {
            "hex" => Self::Hex,
            "rgb" => Self::Rgb,
            "hsl" => Self::Hsl,
            "hsv" => Self::Hsv,
            "cmyk" => Self::Cmyk,
            "xyz" => Self::Xyz,
            "cielab" => Self::Lab,
            "hwb" => Self::Hwb,
            "hcl" => Self::Hcl,
            "name" => Self::Name,
            "lms" => Self::Lms,
            "hunterlab" => Self::HunterLab,
            "oklab" => Self::Oklab,
            "oklch" => Self::Oklch,
            _ => {
                log::error!("Failed to parse notation: {}", s);
                return Err(ColorError::ParsingError(
                    "Failed to get color notation".to_owned(),
                ));
            }
        })
    }
}
