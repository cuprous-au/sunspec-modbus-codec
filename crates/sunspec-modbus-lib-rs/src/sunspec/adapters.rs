use crate::buffer::{ModbusBuffer, write_string, write_u16};
use crate::cursor::Cursor;
use crate::sunspec::models::{
    model_1, model_2, model_3, model_4, model_5, model_6, model_7, model_8, model_10, model_11,
    model_12, model_13, model_15, model_16, model_17, model_18, model_19, model_101, model_102,
    model_103, model_111, model_112, model_113, model_120, model_121, model_122, model_123,
    model_124, model_125, model_126, model_127, model_128, model_129, model_130, model_131,
    model_132, model_133, model_134, model_135, model_136, model_137, model_138, model_139,
    model_140, model_141, model_142, model_143, model_144, model_145, model_160, model_201,
    model_202, model_203, model_204, model_211, model_212, model_213, model_214, model_220,
    model_305, model_306, model_307, model_308, model_401, model_402, model_403, model_404,
    model_501, model_502, model_701, model_703, model_704, model_705, model_706, model_707,
    model_708, model_709, model_710, model_711, model_712, model_713, model_714, model_715,
    model_801, model_802, model_805, model_806, model_807, model_808, model_809, model_63001,
    model_64001, model_64020, model_64101, model_64111, model_64112, model_64410, model_64411,
    model_64412, model_64413, model_64414, model_64415,
};

pub trait SunspecAdapterProvider<'a> {
    fn model_1_adapter(&self) -> Option<&'a dyn model_1::ModelAdapter>;

    fn model_2_adapter(&self) -> Option<&'a dyn model_2::ModelAdapter>;

    fn model_3_adapter(&self) -> Option<&'a dyn model_3::ModelAdapter>;

    fn model_4_adapter(&self) -> Option<&'a dyn model_4::ModelAdapter>;

    fn model_5_adapter(&self) -> Option<&'a dyn model_5::ModelAdapter>;

    fn model_6_adapter(&self) -> Option<&'a dyn model_6::ModelAdapter>;

    fn model_7_adapter(&self) -> Option<&'a dyn model_7::ModelAdapter>;

    fn model_8_adapter(&self) -> Option<&'a dyn model_8::ModelAdapter>;

    fn model_10_adapter(&self) -> Option<&'a dyn model_10::ModelAdapter>;

    fn model_11_adapter(&self) -> Option<&'a dyn model_11::ModelAdapter>;

    fn model_12_adapter(&self) -> Option<&'a dyn model_12::ModelAdapter>;

    fn model_13_adapter(&self) -> Option<&'a dyn model_13::ModelAdapter>;

    fn model_15_adapter(&self) -> Option<&'a dyn model_15::ModelAdapter>;

    fn model_16_adapter(&self) -> Option<&'a dyn model_16::ModelAdapter>;

    fn model_17_adapter(&self) -> Option<&'a dyn model_17::ModelAdapter>;

    fn model_18_adapter(&self) -> Option<&'a dyn model_18::ModelAdapter>;

    fn model_19_adapter(&self) -> Option<&'a dyn model_19::ModelAdapter>;

    fn model_101_adapter(&self) -> Option<&'a dyn model_101::ModelAdapter>;

    fn model_102_adapter(&self) -> Option<&'a dyn model_102::ModelAdapter>;

    fn model_103_adapter(&self) -> Option<&'a dyn model_103::ModelAdapter>;

    fn model_111_adapter(&self) -> Option<&'a dyn model_111::ModelAdapter>;

    fn model_112_adapter(&self) -> Option<&'a dyn model_112::ModelAdapter>;

    fn model_113_adapter(&self) -> Option<&'a dyn model_113::ModelAdapter>;

    fn model_120_adapter(&self) -> Option<&'a dyn model_120::ModelAdapter>;

    fn model_121_adapter(&self) -> Option<&'a dyn model_121::ModelAdapter>;

    fn model_122_adapter(&self) -> Option<&'a dyn model_122::ModelAdapter>;

    fn model_123_adapter(&self) -> Option<&'a dyn model_123::ModelAdapter>;

    fn model_124_adapter(&self) -> Option<&'a dyn model_124::ModelAdapter>;

    fn model_125_adapter(&self) -> Option<&'a dyn model_125::ModelAdapter>;

    fn model_126_adapter(&self) -> Option<&'a dyn model_126::ModelAdapter>;

    fn model_127_adapter(&self) -> Option<&'a dyn model_127::ModelAdapter>;

    fn model_128_adapter(&self) -> Option<&'a dyn model_128::ModelAdapter>;

    fn model_129_adapter(&self) -> Option<&'a dyn model_129::ModelAdapter>;

    fn model_130_adapter(&self) -> Option<&'a dyn model_130::ModelAdapter>;

    fn model_131_adapter(&self) -> Option<&'a dyn model_131::ModelAdapter>;

    fn model_132_adapter(&self) -> Option<&'a dyn model_132::ModelAdapter>;

    fn model_133_adapter(&self) -> Option<&'a dyn model_133::ModelAdapter>;

    fn model_134_adapter(&self) -> Option<&'a dyn model_134::ModelAdapter>;

    fn model_135_adapter(&self) -> Option<&'a dyn model_135::ModelAdapter>;

    fn model_136_adapter(&self) -> Option<&'a dyn model_136::ModelAdapter>;

    fn model_137_adapter(&self) -> Option<&'a dyn model_137::ModelAdapter>;

    fn model_138_adapter(&self) -> Option<&'a dyn model_138::ModelAdapter>;

    fn model_139_adapter(&self) -> Option<&'a dyn model_139::ModelAdapter>;

    fn model_140_adapter(&self) -> Option<&'a dyn model_140::ModelAdapter>;

    fn model_141_adapter(&self) -> Option<&'a dyn model_141::ModelAdapter>;

    fn model_142_adapter(&self) -> Option<&'a dyn model_142::ModelAdapter>;

    fn model_143_adapter(&self) -> Option<&'a dyn model_143::ModelAdapter>;

    fn model_144_adapter(&self) -> Option<&'a dyn model_144::ModelAdapter>;

    fn model_145_adapter(&self) -> Option<&'a dyn model_145::ModelAdapter>;

    fn model_160_adapter(&self) -> Option<&'a dyn model_160::ModelAdapter>;

    fn model_201_adapter(&self) -> Option<&'a dyn model_201::ModelAdapter>;

    fn model_202_adapter(&self) -> Option<&'a dyn model_202::ModelAdapter>;

    fn model_203_adapter(&self) -> Option<&'a dyn model_203::ModelAdapter>;

    fn model_204_adapter(&self) -> Option<&'a dyn model_204::ModelAdapter>;

    fn model_211_adapter(&self) -> Option<&'a dyn model_211::ModelAdapter>;

    fn model_212_adapter(&self) -> Option<&'a dyn model_212::ModelAdapter>;

    fn model_213_adapter(&self) -> Option<&'a dyn model_213::ModelAdapter>;

    fn model_214_adapter(&self) -> Option<&'a dyn model_214::ModelAdapter>;

    fn model_220_adapter(&self) -> Option<&'a dyn model_220::ModelAdapter>;

    fn model_305_adapter(&self) -> Option<&'a dyn model_305::ModelAdapter>;

    fn model_306_adapter(&self) -> Option<&'a dyn model_306::ModelAdapter>;

    fn model_307_adapter(&self) -> Option<&'a dyn model_307::ModelAdapter>;

    fn model_308_adapter(&self) -> Option<&'a dyn model_308::ModelAdapter>;

    fn model_401_adapter(&self) -> Option<&'a dyn model_401::ModelAdapter>;

    fn model_402_adapter(&self) -> Option<&'a dyn model_402::ModelAdapter>;

    fn model_403_adapter(&self) -> Option<&'a dyn model_403::ModelAdapter>;

    fn model_404_adapter(&self) -> Option<&'a dyn model_404::ModelAdapter>;

    fn model_501_adapter(&self) -> Option<&'a dyn model_501::ModelAdapter>;

    fn model_502_adapter(&self) -> Option<&'a dyn model_502::ModelAdapter>;

    fn model_701_adapter(&self) -> Option<&'a dyn model_701::ModelAdapter>;

    fn model_703_adapter(&self) -> Option<&'a dyn model_703::ModelAdapter>;

    fn model_704_adapter(&self) -> Option<&'a dyn model_704::ModelAdapter>;

    fn model_705_adapter(&self) -> Option<&'a dyn model_705::ModelAdapter>;

    fn model_706_adapter(&self) -> Option<&'a dyn model_706::ModelAdapter>;

    fn model_707_adapter(&self) -> Option<&'a dyn model_707::ModelAdapter>;

    fn model_708_adapter(&self) -> Option<&'a dyn model_708::ModelAdapter>;

    fn model_709_adapter(&self) -> Option<&'a dyn model_709::ModelAdapter>;

    fn model_710_adapter(&self) -> Option<&'a dyn model_710::ModelAdapter>;

    fn model_711_adapter(&self) -> Option<&'a dyn model_711::ModelAdapter>;

    fn model_712_adapter(&self) -> Option<&'a dyn model_712::ModelAdapter>;

    fn model_713_adapter(&self) -> Option<&'a dyn model_713::ModelAdapter>;

    fn model_714_adapter(&self) -> Option<&'a dyn model_714::ModelAdapter>;

    fn model_715_adapter(&self) -> Option<&'a dyn model_715::ModelAdapter>;

    fn model_801_adapter(&self) -> Option<&'a dyn model_801::ModelAdapter>;

    fn model_802_adapter(&self) -> Option<&'a dyn model_802::ModelAdapter>;

    fn model_805_adapter(&self) -> Option<&'a dyn model_805::ModelAdapter>;

    fn model_806_adapter(&self) -> Option<&'a dyn model_806::ModelAdapter>;

    fn model_807_adapter(&self) -> Option<&'a dyn model_807::ModelAdapter>;

    fn model_808_adapter(&self) -> Option<&'a dyn model_808::ModelAdapter>;

    fn model_809_adapter(&self) -> Option<&'a dyn model_809::ModelAdapter>;

    fn model_63001_adapter(&self) -> Option<&'a dyn model_63001::ModelAdapter>;

    fn model_64001_adapter(&self) -> Option<&'a dyn model_64001::ModelAdapter>;

    fn model_64020_adapter(&self) -> Option<&'a dyn model_64020::ModelAdapter>;

    fn model_64101_adapter(&self) -> Option<&'a dyn model_64101::ModelAdapter>;

    fn model_64111_adapter(&self) -> Option<&'a dyn model_64111::ModelAdapter>;

    fn model_64112_adapter(&self) -> Option<&'a dyn model_64112::ModelAdapter>;

    fn model_64410_adapter(&self) -> Option<&'a dyn model_64410::ModelAdapter>;

    fn model_64411_adapter(&self) -> Option<&'a dyn model_64411::ModelAdapter>;

    fn model_64412_adapter(&self) -> Option<&'a dyn model_64412::ModelAdapter>;

    fn model_64413_adapter(&self) -> Option<&'a dyn model_64413::ModelAdapter>;

    fn model_64414_adapter(&self) -> Option<&'a dyn model_64414::ModelAdapter>;

    fn model_64415_adapter(&self) -> Option<&'a dyn model_64415::ModelAdapter>;
}

#[derive(Default)]
pub struct SunspecAdapters<'a> {
    pub model_1_adapter: Option<&'a dyn model_1::ModelAdapter>,
    pub model_2_adapter: Option<&'a dyn model_2::ModelAdapter>,
    pub model_3_adapter: Option<&'a dyn model_3::ModelAdapter>,
    pub model_4_adapter: Option<&'a dyn model_4::ModelAdapter>,
    pub model_5_adapter: Option<&'a dyn model_5::ModelAdapter>,
    pub model_6_adapter: Option<&'a dyn model_6::ModelAdapter>,
    pub model_7_adapter: Option<&'a dyn model_7::ModelAdapter>,
    pub model_8_adapter: Option<&'a dyn model_8::ModelAdapter>,
    pub model_10_adapter: Option<&'a dyn model_10::ModelAdapter>,
    pub model_11_adapter: Option<&'a dyn model_11::ModelAdapter>,
    pub model_12_adapter: Option<&'a dyn model_12::ModelAdapter>,
    pub model_13_adapter: Option<&'a dyn model_13::ModelAdapter>,
    pub model_15_adapter: Option<&'a dyn model_15::ModelAdapter>,
    pub model_16_adapter: Option<&'a dyn model_16::ModelAdapter>,
    pub model_17_adapter: Option<&'a dyn model_17::ModelAdapter>,
    pub model_18_adapter: Option<&'a dyn model_18::ModelAdapter>,
    pub model_19_adapter: Option<&'a dyn model_19::ModelAdapter>,
    pub model_101_adapter: Option<&'a dyn model_101::ModelAdapter>,
    pub model_102_adapter: Option<&'a dyn model_102::ModelAdapter>,
    pub model_103_adapter: Option<&'a dyn model_103::ModelAdapter>,
    pub model_111_adapter: Option<&'a dyn model_111::ModelAdapter>,
    pub model_112_adapter: Option<&'a dyn model_112::ModelAdapter>,
    pub model_113_adapter: Option<&'a dyn model_113::ModelAdapter>,
    pub model_120_adapter: Option<&'a dyn model_120::ModelAdapter>,
    pub model_121_adapter: Option<&'a dyn model_121::ModelAdapter>,
    pub model_122_adapter: Option<&'a dyn model_122::ModelAdapter>,
    pub model_123_adapter: Option<&'a dyn model_123::ModelAdapter>,
    pub model_124_adapter: Option<&'a dyn model_124::ModelAdapter>,
    pub model_125_adapter: Option<&'a dyn model_125::ModelAdapter>,
    pub model_126_adapter: Option<&'a dyn model_126::ModelAdapter>,
    pub model_127_adapter: Option<&'a dyn model_127::ModelAdapter>,
    pub model_128_adapter: Option<&'a dyn model_128::ModelAdapter>,
    pub model_129_adapter: Option<&'a dyn model_129::ModelAdapter>,
    pub model_130_adapter: Option<&'a dyn model_130::ModelAdapter>,
    pub model_131_adapter: Option<&'a dyn model_131::ModelAdapter>,
    pub model_132_adapter: Option<&'a dyn model_132::ModelAdapter>,
    pub model_133_adapter: Option<&'a dyn model_133::ModelAdapter>,
    pub model_134_adapter: Option<&'a dyn model_134::ModelAdapter>,
    pub model_135_adapter: Option<&'a dyn model_135::ModelAdapter>,
    pub model_136_adapter: Option<&'a dyn model_136::ModelAdapter>,
    pub model_137_adapter: Option<&'a dyn model_137::ModelAdapter>,
    pub model_138_adapter: Option<&'a dyn model_138::ModelAdapter>,
    pub model_139_adapter: Option<&'a dyn model_139::ModelAdapter>,
    pub model_140_adapter: Option<&'a dyn model_140::ModelAdapter>,
    pub model_141_adapter: Option<&'a dyn model_141::ModelAdapter>,
    pub model_142_adapter: Option<&'a dyn model_142::ModelAdapter>,
    pub model_143_adapter: Option<&'a dyn model_143::ModelAdapter>,
    pub model_144_adapter: Option<&'a dyn model_144::ModelAdapter>,
    pub model_145_adapter: Option<&'a dyn model_145::ModelAdapter>,
    pub model_160_adapter: Option<&'a dyn model_160::ModelAdapter>,
    pub model_201_adapter: Option<&'a dyn model_201::ModelAdapter>,
    pub model_202_adapter: Option<&'a dyn model_202::ModelAdapter>,
    pub model_203_adapter: Option<&'a dyn model_203::ModelAdapter>,
    pub model_204_adapter: Option<&'a dyn model_204::ModelAdapter>,
    pub model_211_adapter: Option<&'a dyn model_211::ModelAdapter>,
    pub model_212_adapter: Option<&'a dyn model_212::ModelAdapter>,
    pub model_213_adapter: Option<&'a dyn model_213::ModelAdapter>,
    pub model_214_adapter: Option<&'a dyn model_214::ModelAdapter>,
    pub model_220_adapter: Option<&'a dyn model_220::ModelAdapter>,
    pub model_305_adapter: Option<&'a dyn model_305::ModelAdapter>,
    pub model_306_adapter: Option<&'a dyn model_306::ModelAdapter>,
    pub model_307_adapter: Option<&'a dyn model_307::ModelAdapter>,
    pub model_308_adapter: Option<&'a dyn model_308::ModelAdapter>,
    pub model_401_adapter: Option<&'a dyn model_401::ModelAdapter>,
    pub model_402_adapter: Option<&'a dyn model_402::ModelAdapter>,
    pub model_403_adapter: Option<&'a dyn model_403::ModelAdapter>,
    pub model_404_adapter: Option<&'a dyn model_404::ModelAdapter>,
    pub model_501_adapter: Option<&'a dyn model_501::ModelAdapter>,
    pub model_502_adapter: Option<&'a dyn model_502::ModelAdapter>,
    pub model_701_adapter: Option<&'a dyn model_701::ModelAdapter>,
    pub model_703_adapter: Option<&'a dyn model_703::ModelAdapter>,
    pub model_704_adapter: Option<&'a dyn model_704::ModelAdapter>,
    pub model_705_adapter: Option<&'a dyn model_705::ModelAdapter>,
    pub model_706_adapter: Option<&'a dyn model_706::ModelAdapter>,
    pub model_707_adapter: Option<&'a dyn model_707::ModelAdapter>,
    pub model_708_adapter: Option<&'a dyn model_708::ModelAdapter>,
    pub model_709_adapter: Option<&'a dyn model_709::ModelAdapter>,
    pub model_710_adapter: Option<&'a dyn model_710::ModelAdapter>,
    pub model_711_adapter: Option<&'a dyn model_711::ModelAdapter>,
    pub model_712_adapter: Option<&'a dyn model_712::ModelAdapter>,
    pub model_713_adapter: Option<&'a dyn model_713::ModelAdapter>,
    pub model_714_adapter: Option<&'a dyn model_714::ModelAdapter>,
    pub model_715_adapter: Option<&'a dyn model_715::ModelAdapter>,
    pub model_801_adapter: Option<&'a dyn model_801::ModelAdapter>,
    pub model_802_adapter: Option<&'a dyn model_802::ModelAdapter>,
    pub model_805_adapter: Option<&'a dyn model_805::ModelAdapter>,
    pub model_806_adapter: Option<&'a dyn model_806::ModelAdapter>,
    pub model_807_adapter: Option<&'a dyn model_807::ModelAdapter>,
    pub model_808_adapter: Option<&'a dyn model_808::ModelAdapter>,
    pub model_809_adapter: Option<&'a dyn model_809::ModelAdapter>,
    pub model_63001_adapter: Option<&'a dyn model_63001::ModelAdapter>,
    pub model_64001_adapter: Option<&'a dyn model_64001::ModelAdapter>,
    pub model_64020_adapter: Option<&'a dyn model_64020::ModelAdapter>,
    pub model_64101_adapter: Option<&'a dyn model_64101::ModelAdapter>,
    pub model_64111_adapter: Option<&'a dyn model_64111::ModelAdapter>,
    pub model_64112_adapter: Option<&'a dyn model_64112::ModelAdapter>,
    pub model_64410_adapter: Option<&'a dyn model_64410::ModelAdapter>,
    pub model_64411_adapter: Option<&'a dyn model_64411::ModelAdapter>,
    pub model_64412_adapter: Option<&'a dyn model_64412::ModelAdapter>,
    pub model_64413_adapter: Option<&'a dyn model_64413::ModelAdapter>,
    pub model_64414_adapter: Option<&'a dyn model_64414::ModelAdapter>,
    pub model_64415_adapter: Option<&'a dyn model_64415::ModelAdapter>,
}

impl<'a> SunspecAdapterProvider<'a> for SunspecAdapters<'a> {
    fn model_1_adapter(&self) -> Option<&'a dyn model_1::ModelAdapter> {
        self.model_1_adapter
    }

    fn model_2_adapter(&self) -> Option<&'a dyn model_2::ModelAdapter> {
        self.model_2_adapter
    }

    fn model_3_adapter(&self) -> Option<&'a dyn model_3::ModelAdapter> {
        self.model_3_adapter
    }

    fn model_4_adapter(&self) -> Option<&'a dyn model_4::ModelAdapter> {
        self.model_4_adapter
    }

    fn model_5_adapter(&self) -> Option<&'a dyn model_5::ModelAdapter> {
        self.model_5_adapter
    }

    fn model_6_adapter(&self) -> Option<&'a dyn model_6::ModelAdapter> {
        self.model_6_adapter
    }

    fn model_7_adapter(&self) -> Option<&'a dyn model_7::ModelAdapter> {
        self.model_7_adapter
    }

    fn model_8_adapter(&self) -> Option<&'a dyn model_8::ModelAdapter> {
        self.model_8_adapter
    }

    fn model_10_adapter(&self) -> Option<&'a dyn model_10::ModelAdapter> {
        self.model_10_adapter
    }

    fn model_11_adapter(&self) -> Option<&'a dyn model_11::ModelAdapter> {
        self.model_11_adapter
    }

    fn model_12_adapter(&self) -> Option<&'a dyn model_12::ModelAdapter> {
        self.model_12_adapter
    }

    fn model_13_adapter(&self) -> Option<&'a dyn model_13::ModelAdapter> {
        self.model_13_adapter
    }

    fn model_15_adapter(&self) -> Option<&'a dyn model_15::ModelAdapter> {
        self.model_15_adapter
    }

    fn model_16_adapter(&self) -> Option<&'a dyn model_16::ModelAdapter> {
        self.model_16_adapter
    }

    fn model_17_adapter(&self) -> Option<&'a dyn model_17::ModelAdapter> {
        self.model_17_adapter
    }

    fn model_18_adapter(&self) -> Option<&'a dyn model_18::ModelAdapter> {
        self.model_18_adapter
    }

    fn model_19_adapter(&self) -> Option<&'a dyn model_19::ModelAdapter> {
        self.model_19_adapter
    }

    fn model_101_adapter(&self) -> Option<&'a dyn model_101::ModelAdapter> {
        self.model_101_adapter
    }

    fn model_102_adapter(&self) -> Option<&'a dyn model_102::ModelAdapter> {
        self.model_102_adapter
    }

    fn model_103_adapter(&self) -> Option<&'a dyn model_103::ModelAdapter> {
        self.model_103_adapter
    }

    fn model_111_adapter(&self) -> Option<&'a dyn model_111::ModelAdapter> {
        self.model_111_adapter
    }

    fn model_112_adapter(&self) -> Option<&'a dyn model_112::ModelAdapter> {
        self.model_112_adapter
    }

    fn model_113_adapter(&self) -> Option<&'a dyn model_113::ModelAdapter> {
        self.model_113_adapter
    }

    fn model_120_adapter(&self) -> Option<&'a dyn model_120::ModelAdapter> {
        self.model_120_adapter
    }

    fn model_121_adapter(&self) -> Option<&'a dyn model_121::ModelAdapter> {
        self.model_121_adapter
    }

    fn model_122_adapter(&self) -> Option<&'a dyn model_122::ModelAdapter> {
        self.model_122_adapter
    }

    fn model_123_adapter(&self) -> Option<&'a dyn model_123::ModelAdapter> {
        self.model_123_adapter
    }

    fn model_124_adapter(&self) -> Option<&'a dyn model_124::ModelAdapter> {
        self.model_124_adapter
    }

    fn model_125_adapter(&self) -> Option<&'a dyn model_125::ModelAdapter> {
        self.model_125_adapter
    }

    fn model_126_adapter(&self) -> Option<&'a dyn model_126::ModelAdapter> {
        self.model_126_adapter
    }

    fn model_127_adapter(&self) -> Option<&'a dyn model_127::ModelAdapter> {
        self.model_127_adapter
    }

    fn model_128_adapter(&self) -> Option<&'a dyn model_128::ModelAdapter> {
        self.model_128_adapter
    }

    fn model_129_adapter(&self) -> Option<&'a dyn model_129::ModelAdapter> {
        self.model_129_adapter
    }

    fn model_130_adapter(&self) -> Option<&'a dyn model_130::ModelAdapter> {
        self.model_130_adapter
    }

    fn model_131_adapter(&self) -> Option<&'a dyn model_131::ModelAdapter> {
        self.model_131_adapter
    }

    fn model_132_adapter(&self) -> Option<&'a dyn model_132::ModelAdapter> {
        self.model_132_adapter
    }

    fn model_133_adapter(&self) -> Option<&'a dyn model_133::ModelAdapter> {
        self.model_133_adapter
    }

    fn model_134_adapter(&self) -> Option<&'a dyn model_134::ModelAdapter> {
        self.model_134_adapter
    }

    fn model_135_adapter(&self) -> Option<&'a dyn model_135::ModelAdapter> {
        self.model_135_adapter
    }

    fn model_136_adapter(&self) -> Option<&'a dyn model_136::ModelAdapter> {
        self.model_136_adapter
    }

    fn model_137_adapter(&self) -> Option<&'a dyn model_137::ModelAdapter> {
        self.model_137_adapter
    }

    fn model_138_adapter(&self) -> Option<&'a dyn model_138::ModelAdapter> {
        self.model_138_adapter
    }

    fn model_139_adapter(&self) -> Option<&'a dyn model_139::ModelAdapter> {
        self.model_139_adapter
    }

    fn model_140_adapter(&self) -> Option<&'a dyn model_140::ModelAdapter> {
        self.model_140_adapter
    }

    fn model_141_adapter(&self) -> Option<&'a dyn model_141::ModelAdapter> {
        self.model_141_adapter
    }

    fn model_142_adapter(&self) -> Option<&'a dyn model_142::ModelAdapter> {
        self.model_142_adapter
    }

    fn model_143_adapter(&self) -> Option<&'a dyn model_143::ModelAdapter> {
        self.model_143_adapter
    }

    fn model_144_adapter(&self) -> Option<&'a dyn model_144::ModelAdapter> {
        self.model_144_adapter
    }

    fn model_145_adapter(&self) -> Option<&'a dyn model_145::ModelAdapter> {
        self.model_145_adapter
    }

    fn model_160_adapter(&self) -> Option<&'a dyn model_160::ModelAdapter> {
        self.model_160_adapter
    }

    fn model_201_adapter(&self) -> Option<&'a dyn model_201::ModelAdapter> {
        self.model_201_adapter
    }

    fn model_202_adapter(&self) -> Option<&'a dyn model_202::ModelAdapter> {
        self.model_202_adapter
    }

    fn model_203_adapter(&self) -> Option<&'a dyn model_203::ModelAdapter> {
        self.model_203_adapter
    }

    fn model_204_adapter(&self) -> Option<&'a dyn model_204::ModelAdapter> {
        self.model_204_adapter
    }

    fn model_211_adapter(&self) -> Option<&'a dyn model_211::ModelAdapter> {
        self.model_211_adapter
    }

    fn model_212_adapter(&self) -> Option<&'a dyn model_212::ModelAdapter> {
        self.model_212_adapter
    }

    fn model_213_adapter(&self) -> Option<&'a dyn model_213::ModelAdapter> {
        self.model_213_adapter
    }

    fn model_214_adapter(&self) -> Option<&'a dyn model_214::ModelAdapter> {
        self.model_214_adapter
    }

    fn model_220_adapter(&self) -> Option<&'a dyn model_220::ModelAdapter> {
        self.model_220_adapter
    }

    fn model_305_adapter(&self) -> Option<&'a dyn model_305::ModelAdapter> {
        self.model_305_adapter
    }

    fn model_306_adapter(&self) -> Option<&'a dyn model_306::ModelAdapter> {
        self.model_306_adapter
    }

    fn model_307_adapter(&self) -> Option<&'a dyn model_307::ModelAdapter> {
        self.model_307_adapter
    }

    fn model_308_adapter(&self) -> Option<&'a dyn model_308::ModelAdapter> {
        self.model_308_adapter
    }

    fn model_401_adapter(&self) -> Option<&'a dyn model_401::ModelAdapter> {
        self.model_401_adapter
    }

    fn model_402_adapter(&self) -> Option<&'a dyn model_402::ModelAdapter> {
        self.model_402_adapter
    }

    fn model_403_adapter(&self) -> Option<&'a dyn model_403::ModelAdapter> {
        self.model_403_adapter
    }

    fn model_404_adapter(&self) -> Option<&'a dyn model_404::ModelAdapter> {
        self.model_404_adapter
    }

    fn model_501_adapter(&self) -> Option<&'a dyn model_501::ModelAdapter> {
        self.model_501_adapter
    }

    fn model_502_adapter(&self) -> Option<&'a dyn model_502::ModelAdapter> {
        self.model_502_adapter
    }

    fn model_701_adapter(&self) -> Option<&'a dyn model_701::ModelAdapter> {
        self.model_701_adapter
    }

    fn model_703_adapter(&self) -> Option<&'a dyn model_703::ModelAdapter> {
        self.model_703_adapter
    }

    fn model_704_adapter(&self) -> Option<&'a dyn model_704::ModelAdapter> {
        self.model_704_adapter
    }

    fn model_705_adapter(&self) -> Option<&'a dyn model_705::ModelAdapter> {
        self.model_705_adapter
    }

    fn model_706_adapter(&self) -> Option<&'a dyn model_706::ModelAdapter> {
        self.model_706_adapter
    }

    fn model_707_adapter(&self) -> Option<&'a dyn model_707::ModelAdapter> {
        self.model_707_adapter
    }

    fn model_708_adapter(&self) -> Option<&'a dyn model_708::ModelAdapter> {
        self.model_708_adapter
    }

    fn model_709_adapter(&self) -> Option<&'a dyn model_709::ModelAdapter> {
        self.model_709_adapter
    }

    fn model_710_adapter(&self) -> Option<&'a dyn model_710::ModelAdapter> {
        self.model_710_adapter
    }

    fn model_711_adapter(&self) -> Option<&'a dyn model_711::ModelAdapter> {
        self.model_711_adapter
    }

    fn model_712_adapter(&self) -> Option<&'a dyn model_712::ModelAdapter> {
        self.model_712_adapter
    }

    fn model_713_adapter(&self) -> Option<&'a dyn model_713::ModelAdapter> {
        self.model_713_adapter
    }

    fn model_714_adapter(&self) -> Option<&'a dyn model_714::ModelAdapter> {
        self.model_714_adapter
    }

    fn model_715_adapter(&self) -> Option<&'a dyn model_715::ModelAdapter> {
        self.model_715_adapter
    }

    fn model_801_adapter(&self) -> Option<&'a dyn model_801::ModelAdapter> {
        self.model_801_adapter
    }

    fn model_802_adapter(&self) -> Option<&'a dyn model_802::ModelAdapter> {
        self.model_802_adapter
    }

    fn model_805_adapter(&self) -> Option<&'a dyn model_805::ModelAdapter> {
        self.model_805_adapter
    }

    fn model_806_adapter(&self) -> Option<&'a dyn model_806::ModelAdapter> {
        self.model_806_adapter
    }

    fn model_807_adapter(&self) -> Option<&'a dyn model_807::ModelAdapter> {
        self.model_807_adapter
    }

    fn model_808_adapter(&self) -> Option<&'a dyn model_808::ModelAdapter> {
        self.model_808_adapter
    }

    fn model_809_adapter(&self) -> Option<&'a dyn model_809::ModelAdapter> {
        self.model_809_adapter
    }

    fn model_63001_adapter(&self) -> Option<&'a dyn model_63001::ModelAdapter> {
        self.model_63001_adapter
    }

    fn model_64001_adapter(&self) -> Option<&'a dyn model_64001::ModelAdapter> {
        self.model_64001_adapter
    }

    fn model_64020_adapter(&self) -> Option<&'a dyn model_64020::ModelAdapter> {
        self.model_64020_adapter
    }

    fn model_64101_adapter(&self) -> Option<&'a dyn model_64101::ModelAdapter> {
        self.model_64101_adapter
    }

    fn model_64111_adapter(&self) -> Option<&'a dyn model_64111::ModelAdapter> {
        self.model_64111_adapter
    }

    fn model_64112_adapter(&self) -> Option<&'a dyn model_64112::ModelAdapter> {
        self.model_64112_adapter
    }

    fn model_64410_adapter(&self) -> Option<&'a dyn model_64410::ModelAdapter> {
        self.model_64410_adapter
    }

    fn model_64411_adapter(&self) -> Option<&'a dyn model_64411::ModelAdapter> {
        self.model_64411_adapter
    }

    fn model_64412_adapter(&self) -> Option<&'a dyn model_64412::ModelAdapter> {
        self.model_64412_adapter
    }

    fn model_64413_adapter(&self) -> Option<&'a dyn model_64413::ModelAdapter> {
        self.model_64413_adapter
    }

    fn model_64414_adapter(&self) -> Option<&'a dyn model_64414::ModelAdapter> {
        self.model_64414_adapter
    }

    fn model_64415_adapter(&self) -> Option<&'a dyn model_64415::ModelAdapter> {
        self.model_64415_adapter
    }
}

#[repr(C)]
pub struct SunspecExternalAdapters<'a> {
    pub model_1_callback_adapter: Option<&'a model_1::Model1CallbackAdapter>,
    pub model_1_stateful_adapter: Option<&'a model_1::Model1StatefulAdapter>,
    pub model_2_callback_adapter: Option<&'a model_2::Model2CallbackAdapter>,
    pub model_2_stateful_adapter: Option<&'a model_2::Model2StatefulAdapter>,
    pub model_3_callback_adapter: Option<&'a model_3::Model3CallbackAdapter>,
    pub model_3_stateful_adapter: Option<&'a model_3::Model3StatefulAdapter>,
    pub model_4_callback_adapter: Option<&'a model_4::Model4CallbackAdapter>,
    pub model_4_stateful_adapter: Option<&'a model_4::Model4StatefulAdapter>,
    pub model_5_callback_adapter: Option<&'a model_5::Model5CallbackAdapter>,
    pub model_5_stateful_adapter: Option<&'a model_5::Model5StatefulAdapter>,
    pub model_6_callback_adapter: Option<&'a model_6::Model6CallbackAdapter>,
    pub model_6_stateful_adapter: Option<&'a model_6::Model6StatefulAdapter>,
    pub model_7_callback_adapter: Option<&'a model_7::Model7CallbackAdapter>,
    pub model_7_stateful_adapter: Option<&'a model_7::Model7StatefulAdapter>,
    pub model_8_callback_adapter: Option<&'a model_8::Model8CallbackAdapter>,
    pub model_8_stateful_adapter: Option<&'a model_8::Model8StatefulAdapter>,
    pub model_10_callback_adapter: Option<&'a model_10::Model10CallbackAdapter>,
    pub model_10_stateful_adapter: Option<&'a model_10::Model10StatefulAdapter>,
    pub model_11_callback_adapter: Option<&'a model_11::Model11CallbackAdapter>,
    pub model_11_stateful_adapter: Option<&'a model_11::Model11StatefulAdapter>,
    pub model_12_callback_adapter: Option<&'a model_12::Model12CallbackAdapter>,
    pub model_12_stateful_adapter: Option<&'a model_12::Model12StatefulAdapter>,
    pub model_13_callback_adapter: Option<&'a model_13::Model13CallbackAdapter>,
    pub model_13_stateful_adapter: Option<&'a model_13::Model13StatefulAdapter>,
    pub model_15_callback_adapter: Option<&'a model_15::Model15CallbackAdapter>,
    pub model_15_stateful_adapter: Option<&'a model_15::Model15StatefulAdapter>,
    pub model_16_callback_adapter: Option<&'a model_16::Model16CallbackAdapter>,
    pub model_16_stateful_adapter: Option<&'a model_16::Model16StatefulAdapter>,
    pub model_17_callback_adapter: Option<&'a model_17::Model17CallbackAdapter>,
    pub model_17_stateful_adapter: Option<&'a model_17::Model17StatefulAdapter>,
    pub model_18_callback_adapter: Option<&'a model_18::Model18CallbackAdapter>,
    pub model_18_stateful_adapter: Option<&'a model_18::Model18StatefulAdapter>,
    pub model_19_callback_adapter: Option<&'a model_19::Model19CallbackAdapter>,
    pub model_19_stateful_adapter: Option<&'a model_19::Model19StatefulAdapter>,
    pub model_101_callback_adapter: Option<&'a model_101::Model101CallbackAdapter>,
    pub model_101_stateful_adapter: Option<&'a model_101::Model101StatefulAdapter>,
    pub model_102_callback_adapter: Option<&'a model_102::Model102CallbackAdapter>,
    pub model_102_stateful_adapter: Option<&'a model_102::Model102StatefulAdapter>,
    pub model_103_callback_adapter: Option<&'a model_103::Model103CallbackAdapter>,
    pub model_103_stateful_adapter: Option<&'a model_103::Model103StatefulAdapter>,
    pub model_111_callback_adapter: Option<&'a model_111::Model111CallbackAdapter>,
    pub model_111_stateful_adapter: Option<&'a model_111::Model111StatefulAdapter>,
    pub model_112_callback_adapter: Option<&'a model_112::Model112CallbackAdapter>,
    pub model_112_stateful_adapter: Option<&'a model_112::Model112StatefulAdapter>,
    pub model_113_callback_adapter: Option<&'a model_113::Model113CallbackAdapter>,
    pub model_113_stateful_adapter: Option<&'a model_113::Model113StatefulAdapter>,
    pub model_120_callback_adapter: Option<&'a model_120::Model120CallbackAdapter>,
    pub model_120_stateful_adapter: Option<&'a model_120::Model120StatefulAdapter>,
    pub model_121_callback_adapter: Option<&'a model_121::Model121CallbackAdapter>,
    pub model_121_stateful_adapter: Option<&'a model_121::Model121StatefulAdapter>,
    pub model_122_callback_adapter: Option<&'a model_122::Model122CallbackAdapter>,
    pub model_122_stateful_adapter: Option<&'a model_122::Model122StatefulAdapter>,
    pub model_123_callback_adapter: Option<&'a model_123::Model123CallbackAdapter>,
    pub model_123_stateful_adapter: Option<&'a model_123::Model123StatefulAdapter>,
    pub model_124_callback_adapter: Option<&'a model_124::Model124CallbackAdapter>,
    pub model_124_stateful_adapter: Option<&'a model_124::Model124StatefulAdapter>,
    pub model_125_callback_adapter: Option<&'a model_125::Model125CallbackAdapter>,
    pub model_125_stateful_adapter: Option<&'a model_125::Model125StatefulAdapter>,
    pub model_126_callback_adapter: Option<&'a model_126::Model126CallbackAdapter>,
    pub model_126_stateful_adapter: Option<&'a model_126::Model126StatefulAdapter>,
    pub model_127_callback_adapter: Option<&'a model_127::Model127CallbackAdapter>,
    pub model_127_stateful_adapter: Option<&'a model_127::Model127StatefulAdapter>,
    pub model_128_callback_adapter: Option<&'a model_128::Model128CallbackAdapter>,
    pub model_128_stateful_adapter: Option<&'a model_128::Model128StatefulAdapter>,
    pub model_129_callback_adapter: Option<&'a model_129::Model129CallbackAdapter>,
    pub model_129_stateful_adapter: Option<&'a model_129::Model129StatefulAdapter>,
    pub model_130_callback_adapter: Option<&'a model_130::Model130CallbackAdapter>,
    pub model_130_stateful_adapter: Option<&'a model_130::Model130StatefulAdapter>,
    pub model_131_callback_adapter: Option<&'a model_131::Model131CallbackAdapter>,
    pub model_131_stateful_adapter: Option<&'a model_131::Model131StatefulAdapter>,
    pub model_132_callback_adapter: Option<&'a model_132::Model132CallbackAdapter>,
    pub model_132_stateful_adapter: Option<&'a model_132::Model132StatefulAdapter>,
    pub model_133_callback_adapter: Option<&'a model_133::Model133CallbackAdapter>,
    pub model_133_stateful_adapter: Option<&'a model_133::Model133StatefulAdapter>,
    pub model_134_callback_adapter: Option<&'a model_134::Model134CallbackAdapter>,
    pub model_134_stateful_adapter: Option<&'a model_134::Model134StatefulAdapter>,
    pub model_135_callback_adapter: Option<&'a model_135::Model135CallbackAdapter>,
    pub model_135_stateful_adapter: Option<&'a model_135::Model135StatefulAdapter>,
    pub model_136_callback_adapter: Option<&'a model_136::Model136CallbackAdapter>,
    pub model_136_stateful_adapter: Option<&'a model_136::Model136StatefulAdapter>,
    pub model_137_callback_adapter: Option<&'a model_137::Model137CallbackAdapter>,
    pub model_137_stateful_adapter: Option<&'a model_137::Model137StatefulAdapter>,
    pub model_138_callback_adapter: Option<&'a model_138::Model138CallbackAdapter>,
    pub model_138_stateful_adapter: Option<&'a model_138::Model138StatefulAdapter>,
    pub model_139_callback_adapter: Option<&'a model_139::Model139CallbackAdapter>,
    pub model_139_stateful_adapter: Option<&'a model_139::Model139StatefulAdapter>,
    pub model_140_callback_adapter: Option<&'a model_140::Model140CallbackAdapter>,
    pub model_140_stateful_adapter: Option<&'a model_140::Model140StatefulAdapter>,
    pub model_141_callback_adapter: Option<&'a model_141::Model141CallbackAdapter>,
    pub model_141_stateful_adapter: Option<&'a model_141::Model141StatefulAdapter>,
    pub model_142_callback_adapter: Option<&'a model_142::Model142CallbackAdapter>,
    pub model_142_stateful_adapter: Option<&'a model_142::Model142StatefulAdapter>,
    pub model_143_callback_adapter: Option<&'a model_143::Model143CallbackAdapter>,
    pub model_143_stateful_adapter: Option<&'a model_143::Model143StatefulAdapter>,
    pub model_144_callback_adapter: Option<&'a model_144::Model144CallbackAdapter>,
    pub model_144_stateful_adapter: Option<&'a model_144::Model144StatefulAdapter>,
    pub model_145_callback_adapter: Option<&'a model_145::Model145CallbackAdapter>,
    pub model_145_stateful_adapter: Option<&'a model_145::Model145StatefulAdapter>,
    pub model_160_callback_adapter: Option<&'a model_160::Model160CallbackAdapter>,
    pub model_160_stateful_adapter: Option<&'a model_160::Model160StatefulAdapter>,
    pub model_201_callback_adapter: Option<&'a model_201::Model201CallbackAdapter>,
    pub model_201_stateful_adapter: Option<&'a model_201::Model201StatefulAdapter>,
    pub model_202_callback_adapter: Option<&'a model_202::Model202CallbackAdapter>,
    pub model_202_stateful_adapter: Option<&'a model_202::Model202StatefulAdapter>,
    pub model_203_callback_adapter: Option<&'a model_203::Model203CallbackAdapter>,
    pub model_203_stateful_adapter: Option<&'a model_203::Model203StatefulAdapter>,
    pub model_204_callback_adapter: Option<&'a model_204::Model204CallbackAdapter>,
    pub model_204_stateful_adapter: Option<&'a model_204::Model204StatefulAdapter>,
    pub model_211_callback_adapter: Option<&'a model_211::Model211CallbackAdapter>,
    pub model_211_stateful_adapter: Option<&'a model_211::Model211StatefulAdapter>,
    pub model_212_callback_adapter: Option<&'a model_212::Model212CallbackAdapter>,
    pub model_212_stateful_adapter: Option<&'a model_212::Model212StatefulAdapter>,
    pub model_213_callback_adapter: Option<&'a model_213::Model213CallbackAdapter>,
    pub model_213_stateful_adapter: Option<&'a model_213::Model213StatefulAdapter>,
    pub model_214_callback_adapter: Option<&'a model_214::Model214CallbackAdapter>,
    pub model_214_stateful_adapter: Option<&'a model_214::Model214StatefulAdapter>,
    pub model_220_callback_adapter: Option<&'a model_220::Model220CallbackAdapter>,
    pub model_220_stateful_adapter: Option<&'a model_220::Model220StatefulAdapter>,
    pub model_305_callback_adapter: Option<&'a model_305::Model305CallbackAdapter>,
    pub model_305_stateful_adapter: Option<&'a model_305::Model305StatefulAdapter>,
    pub model_306_callback_adapter: Option<&'a model_306::Model306CallbackAdapter>,
    pub model_306_stateful_adapter: Option<&'a model_306::Model306StatefulAdapter>,
    pub model_307_callback_adapter: Option<&'a model_307::Model307CallbackAdapter>,
    pub model_307_stateful_adapter: Option<&'a model_307::Model307StatefulAdapter>,
    pub model_308_callback_adapter: Option<&'a model_308::Model308CallbackAdapter>,
    pub model_308_stateful_adapter: Option<&'a model_308::Model308StatefulAdapter>,
    pub model_401_callback_adapter: Option<&'a model_401::Model401CallbackAdapter>,
    pub model_401_stateful_adapter: Option<&'a model_401::Model401StatefulAdapter>,
    pub model_402_callback_adapter: Option<&'a model_402::Model402CallbackAdapter>,
    pub model_402_stateful_adapter: Option<&'a model_402::Model402StatefulAdapter>,
    pub model_403_callback_adapter: Option<&'a model_403::Model403CallbackAdapter>,
    pub model_403_stateful_adapter: Option<&'a model_403::Model403StatefulAdapter>,
    pub model_404_callback_adapter: Option<&'a model_404::Model404CallbackAdapter>,
    pub model_404_stateful_adapter: Option<&'a model_404::Model404StatefulAdapter>,
    pub model_501_callback_adapter: Option<&'a model_501::Model501CallbackAdapter>,
    pub model_501_stateful_adapter: Option<&'a model_501::Model501StatefulAdapter>,
    pub model_502_callback_adapter: Option<&'a model_502::Model502CallbackAdapter>,
    pub model_502_stateful_adapter: Option<&'a model_502::Model502StatefulAdapter>,
    pub model_701_callback_adapter: Option<&'a model_701::Model701CallbackAdapter>,
    pub model_701_stateful_adapter: Option<&'a model_701::Model701StatefulAdapter>,
    pub model_703_callback_adapter: Option<&'a model_703::Model703CallbackAdapter>,
    pub model_703_stateful_adapter: Option<&'a model_703::Model703StatefulAdapter>,
    pub model_704_callback_adapter: Option<&'a model_704::Model704CallbackAdapter>,
    pub model_704_stateful_adapter: Option<&'a model_704::Model704StatefulAdapter>,
    pub model_705_callback_adapter: Option<&'a model_705::Model705CallbackAdapter>,
    pub model_706_callback_adapter: Option<&'a model_706::Model706CallbackAdapter>,
    pub model_707_callback_adapter: Option<&'a model_707::Model707CallbackAdapter>,
    pub model_708_callback_adapter: Option<&'a model_708::Model708CallbackAdapter>,
    pub model_709_callback_adapter: Option<&'a model_709::Model709CallbackAdapter>,
    pub model_710_callback_adapter: Option<&'a model_710::Model710CallbackAdapter>,
    pub model_711_callback_adapter: Option<&'a model_711::Model711CallbackAdapter>,
    pub model_712_callback_adapter: Option<&'a model_712::Model712CallbackAdapter>,
    pub model_713_callback_adapter: Option<&'a model_713::Model713CallbackAdapter>,
    pub model_713_stateful_adapter: Option<&'a model_713::Model713StatefulAdapter>,
    pub model_714_callback_adapter: Option<&'a model_714::Model714CallbackAdapter>,
    pub model_715_callback_adapter: Option<&'a model_715::Model715CallbackAdapter>,
    pub model_715_stateful_adapter: Option<&'a model_715::Model715StatefulAdapter>,
    pub model_801_callback_adapter: Option<&'a model_801::Model801CallbackAdapter>,
    pub model_801_stateful_adapter: Option<&'a model_801::Model801StatefulAdapter>,
    pub model_802_callback_adapter: Option<&'a model_802::Model802CallbackAdapter>,
    pub model_802_stateful_adapter: Option<&'a model_802::Model802StatefulAdapter>,
    pub model_805_callback_adapter: Option<&'a model_805::Model805CallbackAdapter>,
    pub model_805_stateful_adapter: Option<&'a model_805::Model805StatefulAdapter>,
    pub model_806_callback_adapter: Option<&'a model_806::Model806CallbackAdapter>,
    pub model_806_stateful_adapter: Option<&'a model_806::Model806StatefulAdapter>,
    pub model_807_callback_adapter: Option<&'a model_807::Model807CallbackAdapter>,
    pub model_807_stateful_adapter: Option<&'a model_807::Model807StatefulAdapter>,
    pub model_808_callback_adapter: Option<&'a model_808::Model808CallbackAdapter>,
    pub model_808_stateful_adapter: Option<&'a model_808::Model808StatefulAdapter>,
    pub model_809_callback_adapter: Option<&'a model_809::Model809CallbackAdapter>,
    pub model_809_stateful_adapter: Option<&'a model_809::Model809StatefulAdapter>,
    pub model_63001_callback_adapter: Option<&'a model_63001::Model63001CallbackAdapter>,
    pub model_63001_stateful_adapter: Option<&'a model_63001::Model63001StatefulAdapter>,
    pub model_64001_callback_adapter: Option<&'a model_64001::Model64001CallbackAdapter>,
    pub model_64001_stateful_adapter: Option<&'a model_64001::Model64001StatefulAdapter>,
    pub model_64020_callback_adapter: Option<&'a model_64020::Model64020CallbackAdapter>,
    pub model_64020_stateful_adapter: Option<&'a model_64020::Model64020StatefulAdapter>,
    pub model_64101_callback_adapter: Option<&'a model_64101::Model64101CallbackAdapter>,
    pub model_64101_stateful_adapter: Option<&'a model_64101::Model64101StatefulAdapter>,
    pub model_64111_callback_adapter: Option<&'a model_64111::Model64111CallbackAdapter>,
    pub model_64111_stateful_adapter: Option<&'a model_64111::Model64111StatefulAdapter>,
    pub model_64112_callback_adapter: Option<&'a model_64112::Model64112CallbackAdapter>,
    pub model_64112_stateful_adapter: Option<&'a model_64112::Model64112StatefulAdapter>,
    pub model_64410_callback_adapter: Option<&'a model_64410::Model64410CallbackAdapter>,
    pub model_64411_callback_adapter: Option<&'a model_64411::Model64411CallbackAdapter>,
    pub model_64412_callback_adapter: Option<&'a model_64412::Model64412CallbackAdapter>,
    pub model_64412_stateful_adapter: Option<&'a model_64412::Model64412StatefulAdapter>,
    pub model_64413_callback_adapter: Option<&'a model_64413::Model64413CallbackAdapter>,
    pub model_64414_callback_adapter: Option<&'a model_64414::Model64414CallbackAdapter>,
    pub model_64414_stateful_adapter: Option<&'a model_64414::Model64414StatefulAdapter>,
    pub model_64415_callback_adapter: Option<&'a model_64415::Model64415CallbackAdapter>,
    pub model_64415_stateful_adapter: Option<&'a model_64415::Model64415StatefulAdapter>,
}

impl<'a> SunspecAdapterProvider<'a> for SunspecExternalAdapters<'a> {
    fn model_1_adapter(&self) -> Option<&'a dyn model_1::ModelAdapter> {
        self.model_1_callback_adapter
            .map(|a| a as &'a dyn model_1::ModelAdapter)
            .or(self
                .model_1_stateful_adapter
                .map(|a| a as &'a dyn model_1::ModelAdapter))
    }

    fn model_2_adapter(&self) -> Option<&'a dyn model_2::ModelAdapter> {
        self.model_2_callback_adapter
            .map(|a| a as &'a dyn model_2::ModelAdapter)
            .or(self
                .model_2_stateful_adapter
                .map(|a| a as &'a dyn model_2::ModelAdapter))
    }

    fn model_3_adapter(&self) -> Option<&'a dyn model_3::ModelAdapter> {
        self.model_3_callback_adapter
            .map(|a| a as &'a dyn model_3::ModelAdapter)
            .or(self
                .model_3_stateful_adapter
                .map(|a| a as &'a dyn model_3::ModelAdapter))
    }

    fn model_4_adapter(&self) -> Option<&'a dyn model_4::ModelAdapter> {
        self.model_4_callback_adapter
            .map(|a| a as &'a dyn model_4::ModelAdapter)
            .or(self
                .model_4_stateful_adapter
                .map(|a| a as &'a dyn model_4::ModelAdapter))
    }

    fn model_5_adapter(&self) -> Option<&'a dyn model_5::ModelAdapter> {
        self.model_5_callback_adapter
            .map(|a| a as &'a dyn model_5::ModelAdapter)
            .or(self
                .model_5_stateful_adapter
                .map(|a| a as &'a dyn model_5::ModelAdapter))
    }

    fn model_6_adapter(&self) -> Option<&'a dyn model_6::ModelAdapter> {
        self.model_6_callback_adapter
            .map(|a| a as &'a dyn model_6::ModelAdapter)
            .or(self
                .model_6_stateful_adapter
                .map(|a| a as &'a dyn model_6::ModelAdapter))
    }

    fn model_7_adapter(&self) -> Option<&'a dyn model_7::ModelAdapter> {
        self.model_7_callback_adapter
            .map(|a| a as &'a dyn model_7::ModelAdapter)
            .or(self
                .model_7_stateful_adapter
                .map(|a| a as &'a dyn model_7::ModelAdapter))
    }

    fn model_8_adapter(&self) -> Option<&'a dyn model_8::ModelAdapter> {
        self.model_8_callback_adapter
            .map(|a| a as &'a dyn model_8::ModelAdapter)
            .or(self
                .model_8_stateful_adapter
                .map(|a| a as &'a dyn model_8::ModelAdapter))
    }

    fn model_10_adapter(&self) -> Option<&'a dyn model_10::ModelAdapter> {
        self.model_10_callback_adapter
            .map(|a| a as &'a dyn model_10::ModelAdapter)
            .or(self
                .model_10_stateful_adapter
                .map(|a| a as &'a dyn model_10::ModelAdapter))
    }

    fn model_11_adapter(&self) -> Option<&'a dyn model_11::ModelAdapter> {
        self.model_11_callback_adapter
            .map(|a| a as &'a dyn model_11::ModelAdapter)
            .or(self
                .model_11_stateful_adapter
                .map(|a| a as &'a dyn model_11::ModelAdapter))
    }

    fn model_12_adapter(&self) -> Option<&'a dyn model_12::ModelAdapter> {
        self.model_12_callback_adapter
            .map(|a| a as &'a dyn model_12::ModelAdapter)
            .or(self
                .model_12_stateful_adapter
                .map(|a| a as &'a dyn model_12::ModelAdapter))
    }

    fn model_13_adapter(&self) -> Option<&'a dyn model_13::ModelAdapter> {
        self.model_13_callback_adapter
            .map(|a| a as &'a dyn model_13::ModelAdapter)
            .or(self
                .model_13_stateful_adapter
                .map(|a| a as &'a dyn model_13::ModelAdapter))
    }

    fn model_15_adapter(&self) -> Option<&'a dyn model_15::ModelAdapter> {
        self.model_15_callback_adapter
            .map(|a| a as &'a dyn model_15::ModelAdapter)
            .or(self
                .model_15_stateful_adapter
                .map(|a| a as &'a dyn model_15::ModelAdapter))
    }

    fn model_16_adapter(&self) -> Option<&'a dyn model_16::ModelAdapter> {
        self.model_16_callback_adapter
            .map(|a| a as &'a dyn model_16::ModelAdapter)
            .or(self
                .model_16_stateful_adapter
                .map(|a| a as &'a dyn model_16::ModelAdapter))
    }

    fn model_17_adapter(&self) -> Option<&'a dyn model_17::ModelAdapter> {
        self.model_17_callback_adapter
            .map(|a| a as &'a dyn model_17::ModelAdapter)
            .or(self
                .model_17_stateful_adapter
                .map(|a| a as &'a dyn model_17::ModelAdapter))
    }

    fn model_18_adapter(&self) -> Option<&'a dyn model_18::ModelAdapter> {
        self.model_18_callback_adapter
            .map(|a| a as &'a dyn model_18::ModelAdapter)
            .or(self
                .model_18_stateful_adapter
                .map(|a| a as &'a dyn model_18::ModelAdapter))
    }

    fn model_19_adapter(&self) -> Option<&'a dyn model_19::ModelAdapter> {
        self.model_19_callback_adapter
            .map(|a| a as &'a dyn model_19::ModelAdapter)
            .or(self
                .model_19_stateful_adapter
                .map(|a| a as &'a dyn model_19::ModelAdapter))
    }

    fn model_101_adapter(&self) -> Option<&'a dyn model_101::ModelAdapter> {
        self.model_101_callback_adapter
            .map(|a| a as &'a dyn model_101::ModelAdapter)
            .or(self
                .model_101_stateful_adapter
                .map(|a| a as &'a dyn model_101::ModelAdapter))
    }

    fn model_102_adapter(&self) -> Option<&'a dyn model_102::ModelAdapter> {
        self.model_102_callback_adapter
            .map(|a| a as &'a dyn model_102::ModelAdapter)
            .or(self
                .model_102_stateful_adapter
                .map(|a| a as &'a dyn model_102::ModelAdapter))
    }

    fn model_103_adapter(&self) -> Option<&'a dyn model_103::ModelAdapter> {
        self.model_103_callback_adapter
            .map(|a| a as &'a dyn model_103::ModelAdapter)
            .or(self
                .model_103_stateful_adapter
                .map(|a| a as &'a dyn model_103::ModelAdapter))
    }

    fn model_111_adapter(&self) -> Option<&'a dyn model_111::ModelAdapter> {
        self.model_111_callback_adapter
            .map(|a| a as &'a dyn model_111::ModelAdapter)
            .or(self
                .model_111_stateful_adapter
                .map(|a| a as &'a dyn model_111::ModelAdapter))
    }

    fn model_112_adapter(&self) -> Option<&'a dyn model_112::ModelAdapter> {
        self.model_112_callback_adapter
            .map(|a| a as &'a dyn model_112::ModelAdapter)
            .or(self
                .model_112_stateful_adapter
                .map(|a| a as &'a dyn model_112::ModelAdapter))
    }

    fn model_113_adapter(&self) -> Option<&'a dyn model_113::ModelAdapter> {
        self.model_113_callback_adapter
            .map(|a| a as &'a dyn model_113::ModelAdapter)
            .or(self
                .model_113_stateful_adapter
                .map(|a| a as &'a dyn model_113::ModelAdapter))
    }

    fn model_120_adapter(&self) -> Option<&'a dyn model_120::ModelAdapter> {
        self.model_120_callback_adapter
            .map(|a| a as &'a dyn model_120::ModelAdapter)
            .or(self
                .model_120_stateful_adapter
                .map(|a| a as &'a dyn model_120::ModelAdapter))
    }

    fn model_121_adapter(&self) -> Option<&'a dyn model_121::ModelAdapter> {
        self.model_121_callback_adapter
            .map(|a| a as &'a dyn model_121::ModelAdapter)
            .or(self
                .model_121_stateful_adapter
                .map(|a| a as &'a dyn model_121::ModelAdapter))
    }

    fn model_122_adapter(&self) -> Option<&'a dyn model_122::ModelAdapter> {
        self.model_122_callback_adapter
            .map(|a| a as &'a dyn model_122::ModelAdapter)
            .or(self
                .model_122_stateful_adapter
                .map(|a| a as &'a dyn model_122::ModelAdapter))
    }

    fn model_123_adapter(&self) -> Option<&'a dyn model_123::ModelAdapter> {
        self.model_123_callback_adapter
            .map(|a| a as &'a dyn model_123::ModelAdapter)
            .or(self
                .model_123_stateful_adapter
                .map(|a| a as &'a dyn model_123::ModelAdapter))
    }

    fn model_124_adapter(&self) -> Option<&'a dyn model_124::ModelAdapter> {
        self.model_124_callback_adapter
            .map(|a| a as &'a dyn model_124::ModelAdapter)
            .or(self
                .model_124_stateful_adapter
                .map(|a| a as &'a dyn model_124::ModelAdapter))
    }

    fn model_125_adapter(&self) -> Option<&'a dyn model_125::ModelAdapter> {
        self.model_125_callback_adapter
            .map(|a| a as &'a dyn model_125::ModelAdapter)
            .or(self
                .model_125_stateful_adapter
                .map(|a| a as &'a dyn model_125::ModelAdapter))
    }

    fn model_126_adapter(&self) -> Option<&'a dyn model_126::ModelAdapter> {
        self.model_126_callback_adapter
            .map(|a| a as &'a dyn model_126::ModelAdapter)
            .or(self
                .model_126_stateful_adapter
                .map(|a| a as &'a dyn model_126::ModelAdapter))
    }

    fn model_127_adapter(&self) -> Option<&'a dyn model_127::ModelAdapter> {
        self.model_127_callback_adapter
            .map(|a| a as &'a dyn model_127::ModelAdapter)
            .or(self
                .model_127_stateful_adapter
                .map(|a| a as &'a dyn model_127::ModelAdapter))
    }

    fn model_128_adapter(&self) -> Option<&'a dyn model_128::ModelAdapter> {
        self.model_128_callback_adapter
            .map(|a| a as &'a dyn model_128::ModelAdapter)
            .or(self
                .model_128_stateful_adapter
                .map(|a| a as &'a dyn model_128::ModelAdapter))
    }

    fn model_129_adapter(&self) -> Option<&'a dyn model_129::ModelAdapter> {
        self.model_129_callback_adapter
            .map(|a| a as &'a dyn model_129::ModelAdapter)
            .or(self
                .model_129_stateful_adapter
                .map(|a| a as &'a dyn model_129::ModelAdapter))
    }

    fn model_130_adapter(&self) -> Option<&'a dyn model_130::ModelAdapter> {
        self.model_130_callback_adapter
            .map(|a| a as &'a dyn model_130::ModelAdapter)
            .or(self
                .model_130_stateful_adapter
                .map(|a| a as &'a dyn model_130::ModelAdapter))
    }

    fn model_131_adapter(&self) -> Option<&'a dyn model_131::ModelAdapter> {
        self.model_131_callback_adapter
            .map(|a| a as &'a dyn model_131::ModelAdapter)
            .or(self
                .model_131_stateful_adapter
                .map(|a| a as &'a dyn model_131::ModelAdapter))
    }

    fn model_132_adapter(&self) -> Option<&'a dyn model_132::ModelAdapter> {
        self.model_132_callback_adapter
            .map(|a| a as &'a dyn model_132::ModelAdapter)
            .or(self
                .model_132_stateful_adapter
                .map(|a| a as &'a dyn model_132::ModelAdapter))
    }

    fn model_133_adapter(&self) -> Option<&'a dyn model_133::ModelAdapter> {
        self.model_133_callback_adapter
            .map(|a| a as &'a dyn model_133::ModelAdapter)
            .or(self
                .model_133_stateful_adapter
                .map(|a| a as &'a dyn model_133::ModelAdapter))
    }

    fn model_134_adapter(&self) -> Option<&'a dyn model_134::ModelAdapter> {
        self.model_134_callback_adapter
            .map(|a| a as &'a dyn model_134::ModelAdapter)
            .or(self
                .model_134_stateful_adapter
                .map(|a| a as &'a dyn model_134::ModelAdapter))
    }

    fn model_135_adapter(&self) -> Option<&'a dyn model_135::ModelAdapter> {
        self.model_135_callback_adapter
            .map(|a| a as &'a dyn model_135::ModelAdapter)
            .or(self
                .model_135_stateful_adapter
                .map(|a| a as &'a dyn model_135::ModelAdapter))
    }

    fn model_136_adapter(&self) -> Option<&'a dyn model_136::ModelAdapter> {
        self.model_136_callback_adapter
            .map(|a| a as &'a dyn model_136::ModelAdapter)
            .or(self
                .model_136_stateful_adapter
                .map(|a| a as &'a dyn model_136::ModelAdapter))
    }

    fn model_137_adapter(&self) -> Option<&'a dyn model_137::ModelAdapter> {
        self.model_137_callback_adapter
            .map(|a| a as &'a dyn model_137::ModelAdapter)
            .or(self
                .model_137_stateful_adapter
                .map(|a| a as &'a dyn model_137::ModelAdapter))
    }

    fn model_138_adapter(&self) -> Option<&'a dyn model_138::ModelAdapter> {
        self.model_138_callback_adapter
            .map(|a| a as &'a dyn model_138::ModelAdapter)
            .or(self
                .model_138_stateful_adapter
                .map(|a| a as &'a dyn model_138::ModelAdapter))
    }

    fn model_139_adapter(&self) -> Option<&'a dyn model_139::ModelAdapter> {
        self.model_139_callback_adapter
            .map(|a| a as &'a dyn model_139::ModelAdapter)
            .or(self
                .model_139_stateful_adapter
                .map(|a| a as &'a dyn model_139::ModelAdapter))
    }

    fn model_140_adapter(&self) -> Option<&'a dyn model_140::ModelAdapter> {
        self.model_140_callback_adapter
            .map(|a| a as &'a dyn model_140::ModelAdapter)
            .or(self
                .model_140_stateful_adapter
                .map(|a| a as &'a dyn model_140::ModelAdapter))
    }

    fn model_141_adapter(&self) -> Option<&'a dyn model_141::ModelAdapter> {
        self.model_141_callback_adapter
            .map(|a| a as &'a dyn model_141::ModelAdapter)
            .or(self
                .model_141_stateful_adapter
                .map(|a| a as &'a dyn model_141::ModelAdapter))
    }

    fn model_142_adapter(&self) -> Option<&'a dyn model_142::ModelAdapter> {
        self.model_142_callback_adapter
            .map(|a| a as &'a dyn model_142::ModelAdapter)
            .or(self
                .model_142_stateful_adapter
                .map(|a| a as &'a dyn model_142::ModelAdapter))
    }

    fn model_143_adapter(&self) -> Option<&'a dyn model_143::ModelAdapter> {
        self.model_143_callback_adapter
            .map(|a| a as &'a dyn model_143::ModelAdapter)
            .or(self
                .model_143_stateful_adapter
                .map(|a| a as &'a dyn model_143::ModelAdapter))
    }

    fn model_144_adapter(&self) -> Option<&'a dyn model_144::ModelAdapter> {
        self.model_144_callback_adapter
            .map(|a| a as &'a dyn model_144::ModelAdapter)
            .or(self
                .model_144_stateful_adapter
                .map(|a| a as &'a dyn model_144::ModelAdapter))
    }

    fn model_145_adapter(&self) -> Option<&'a dyn model_145::ModelAdapter> {
        self.model_145_callback_adapter
            .map(|a| a as &'a dyn model_145::ModelAdapter)
            .or(self
                .model_145_stateful_adapter
                .map(|a| a as &'a dyn model_145::ModelAdapter))
    }

    fn model_160_adapter(&self) -> Option<&'a dyn model_160::ModelAdapter> {
        self.model_160_callback_adapter
            .map(|a| a as &'a dyn model_160::ModelAdapter)
            .or(self
                .model_160_stateful_adapter
                .map(|a| a as &'a dyn model_160::ModelAdapter))
    }

    fn model_201_adapter(&self) -> Option<&'a dyn model_201::ModelAdapter> {
        self.model_201_callback_adapter
            .map(|a| a as &'a dyn model_201::ModelAdapter)
            .or(self
                .model_201_stateful_adapter
                .map(|a| a as &'a dyn model_201::ModelAdapter))
    }

    fn model_202_adapter(&self) -> Option<&'a dyn model_202::ModelAdapter> {
        self.model_202_callback_adapter
            .map(|a| a as &'a dyn model_202::ModelAdapter)
            .or(self
                .model_202_stateful_adapter
                .map(|a| a as &'a dyn model_202::ModelAdapter))
    }

    fn model_203_adapter(&self) -> Option<&'a dyn model_203::ModelAdapter> {
        self.model_203_callback_adapter
            .map(|a| a as &'a dyn model_203::ModelAdapter)
            .or(self
                .model_203_stateful_adapter
                .map(|a| a as &'a dyn model_203::ModelAdapter))
    }

    fn model_204_adapter(&self) -> Option<&'a dyn model_204::ModelAdapter> {
        self.model_204_callback_adapter
            .map(|a| a as &'a dyn model_204::ModelAdapter)
            .or(self
                .model_204_stateful_adapter
                .map(|a| a as &'a dyn model_204::ModelAdapter))
    }

    fn model_211_adapter(&self) -> Option<&'a dyn model_211::ModelAdapter> {
        self.model_211_callback_adapter
            .map(|a| a as &'a dyn model_211::ModelAdapter)
            .or(self
                .model_211_stateful_adapter
                .map(|a| a as &'a dyn model_211::ModelAdapter))
    }

    fn model_212_adapter(&self) -> Option<&'a dyn model_212::ModelAdapter> {
        self.model_212_callback_adapter
            .map(|a| a as &'a dyn model_212::ModelAdapter)
            .or(self
                .model_212_stateful_adapter
                .map(|a| a as &'a dyn model_212::ModelAdapter))
    }

    fn model_213_adapter(&self) -> Option<&'a dyn model_213::ModelAdapter> {
        self.model_213_callback_adapter
            .map(|a| a as &'a dyn model_213::ModelAdapter)
            .or(self
                .model_213_stateful_adapter
                .map(|a| a as &'a dyn model_213::ModelAdapter))
    }

    fn model_214_adapter(&self) -> Option<&'a dyn model_214::ModelAdapter> {
        self.model_214_callback_adapter
            .map(|a| a as &'a dyn model_214::ModelAdapter)
            .or(self
                .model_214_stateful_adapter
                .map(|a| a as &'a dyn model_214::ModelAdapter))
    }

    fn model_220_adapter(&self) -> Option<&'a dyn model_220::ModelAdapter> {
        self.model_220_callback_adapter
            .map(|a| a as &'a dyn model_220::ModelAdapter)
            .or(self
                .model_220_stateful_adapter
                .map(|a| a as &'a dyn model_220::ModelAdapter))
    }

    fn model_305_adapter(&self) -> Option<&'a dyn model_305::ModelAdapter> {
        self.model_305_callback_adapter
            .map(|a| a as &'a dyn model_305::ModelAdapter)
            .or(self
                .model_305_stateful_adapter
                .map(|a| a as &'a dyn model_305::ModelAdapter))
    }

    fn model_306_adapter(&self) -> Option<&'a dyn model_306::ModelAdapter> {
        self.model_306_callback_adapter
            .map(|a| a as &'a dyn model_306::ModelAdapter)
            .or(self
                .model_306_stateful_adapter
                .map(|a| a as &'a dyn model_306::ModelAdapter))
    }

    fn model_307_adapter(&self) -> Option<&'a dyn model_307::ModelAdapter> {
        self.model_307_callback_adapter
            .map(|a| a as &'a dyn model_307::ModelAdapter)
            .or(self
                .model_307_stateful_adapter
                .map(|a| a as &'a dyn model_307::ModelAdapter))
    }

    fn model_308_adapter(&self) -> Option<&'a dyn model_308::ModelAdapter> {
        self.model_308_callback_adapter
            .map(|a| a as &'a dyn model_308::ModelAdapter)
            .or(self
                .model_308_stateful_adapter
                .map(|a| a as &'a dyn model_308::ModelAdapter))
    }

    fn model_401_adapter(&self) -> Option<&'a dyn model_401::ModelAdapter> {
        self.model_401_callback_adapter
            .map(|a| a as &'a dyn model_401::ModelAdapter)
            .or(self
                .model_401_stateful_adapter
                .map(|a| a as &'a dyn model_401::ModelAdapter))
    }

    fn model_402_adapter(&self) -> Option<&'a dyn model_402::ModelAdapter> {
        self.model_402_callback_adapter
            .map(|a| a as &'a dyn model_402::ModelAdapter)
            .or(self
                .model_402_stateful_adapter
                .map(|a| a as &'a dyn model_402::ModelAdapter))
    }

    fn model_403_adapter(&self) -> Option<&'a dyn model_403::ModelAdapter> {
        self.model_403_callback_adapter
            .map(|a| a as &'a dyn model_403::ModelAdapter)
            .or(self
                .model_403_stateful_adapter
                .map(|a| a as &'a dyn model_403::ModelAdapter))
    }

    fn model_404_adapter(&self) -> Option<&'a dyn model_404::ModelAdapter> {
        self.model_404_callback_adapter
            .map(|a| a as &'a dyn model_404::ModelAdapter)
            .or(self
                .model_404_stateful_adapter
                .map(|a| a as &'a dyn model_404::ModelAdapter))
    }

    fn model_501_adapter(&self) -> Option<&'a dyn model_501::ModelAdapter> {
        self.model_501_callback_adapter
            .map(|a| a as &'a dyn model_501::ModelAdapter)
            .or(self
                .model_501_stateful_adapter
                .map(|a| a as &'a dyn model_501::ModelAdapter))
    }

    fn model_502_adapter(&self) -> Option<&'a dyn model_502::ModelAdapter> {
        self.model_502_callback_adapter
            .map(|a| a as &'a dyn model_502::ModelAdapter)
            .or(self
                .model_502_stateful_adapter
                .map(|a| a as &'a dyn model_502::ModelAdapter))
    }

    fn model_701_adapter(&self) -> Option<&'a dyn model_701::ModelAdapter> {
        self.model_701_callback_adapter
            .map(|a| a as &'a dyn model_701::ModelAdapter)
            .or(self
                .model_701_stateful_adapter
                .map(|a| a as &'a dyn model_701::ModelAdapter))
    }

    fn model_703_adapter(&self) -> Option<&'a dyn model_703::ModelAdapter> {
        self.model_703_callback_adapter
            .map(|a| a as &'a dyn model_703::ModelAdapter)
            .or(self
                .model_703_stateful_adapter
                .map(|a| a as &'a dyn model_703::ModelAdapter))
    }

    fn model_704_adapter(&self) -> Option<&'a dyn model_704::ModelAdapter> {
        self.model_704_callback_adapter
            .map(|a| a as &'a dyn model_704::ModelAdapter)
            .or(self
                .model_704_stateful_adapter
                .map(|a| a as &'a dyn model_704::ModelAdapter))
    }

    fn model_705_adapter(&self) -> Option<&'a dyn model_705::ModelAdapter> {
        self.model_705_callback_adapter
            .map(|a| a as &'a dyn model_705::ModelAdapter)
    }

    fn model_706_adapter(&self) -> Option<&'a dyn model_706::ModelAdapter> {
        self.model_706_callback_adapter
            .map(|a| a as &'a dyn model_706::ModelAdapter)
    }

    fn model_707_adapter(&self) -> Option<&'a dyn model_707::ModelAdapter> {
        self.model_707_callback_adapter
            .map(|a| a as &'a dyn model_707::ModelAdapter)
    }

    fn model_708_adapter(&self) -> Option<&'a dyn model_708::ModelAdapter> {
        self.model_708_callback_adapter
            .map(|a| a as &'a dyn model_708::ModelAdapter)
    }

    fn model_709_adapter(&self) -> Option<&'a dyn model_709::ModelAdapter> {
        self.model_709_callback_adapter
            .map(|a| a as &'a dyn model_709::ModelAdapter)
    }

    fn model_710_adapter(&self) -> Option<&'a dyn model_710::ModelAdapter> {
        self.model_710_callback_adapter
            .map(|a| a as &'a dyn model_710::ModelAdapter)
    }

    fn model_711_adapter(&self) -> Option<&'a dyn model_711::ModelAdapter> {
        self.model_711_callback_adapter
            .map(|a| a as &'a dyn model_711::ModelAdapter)
    }

    fn model_712_adapter(&self) -> Option<&'a dyn model_712::ModelAdapter> {
        self.model_712_callback_adapter
            .map(|a| a as &'a dyn model_712::ModelAdapter)
    }

    fn model_713_adapter(&self) -> Option<&'a dyn model_713::ModelAdapter> {
        self.model_713_callback_adapter
            .map(|a| a as &'a dyn model_713::ModelAdapter)
            .or(self
                .model_713_stateful_adapter
                .map(|a| a as &'a dyn model_713::ModelAdapter))
    }

    fn model_714_adapter(&self) -> Option<&'a dyn model_714::ModelAdapter> {
        self.model_714_callback_adapter
            .map(|a| a as &'a dyn model_714::ModelAdapter)
    }

    fn model_715_adapter(&self) -> Option<&'a dyn model_715::ModelAdapter> {
        self.model_715_callback_adapter
            .map(|a| a as &'a dyn model_715::ModelAdapter)
            .or(self
                .model_715_stateful_adapter
                .map(|a| a as &'a dyn model_715::ModelAdapter))
    }

    fn model_801_adapter(&self) -> Option<&'a dyn model_801::ModelAdapter> {
        self.model_801_callback_adapter
            .map(|a| a as &'a dyn model_801::ModelAdapter)
            .or(self
                .model_801_stateful_adapter
                .map(|a| a as &'a dyn model_801::ModelAdapter))
    }

    fn model_802_adapter(&self) -> Option<&'a dyn model_802::ModelAdapter> {
        self.model_802_callback_adapter
            .map(|a| a as &'a dyn model_802::ModelAdapter)
            .or(self
                .model_802_stateful_adapter
                .map(|a| a as &'a dyn model_802::ModelAdapter))
    }

    fn model_805_adapter(&self) -> Option<&'a dyn model_805::ModelAdapter> {
        self.model_805_callback_adapter
            .map(|a| a as &'a dyn model_805::ModelAdapter)
            .or(self
                .model_805_stateful_adapter
                .map(|a| a as &'a dyn model_805::ModelAdapter))
    }

    fn model_806_adapter(&self) -> Option<&'a dyn model_806::ModelAdapter> {
        self.model_806_callback_adapter
            .map(|a| a as &'a dyn model_806::ModelAdapter)
            .or(self
                .model_806_stateful_adapter
                .map(|a| a as &'a dyn model_806::ModelAdapter))
    }

    fn model_807_adapter(&self) -> Option<&'a dyn model_807::ModelAdapter> {
        self.model_807_callback_adapter
            .map(|a| a as &'a dyn model_807::ModelAdapter)
            .or(self
                .model_807_stateful_adapter
                .map(|a| a as &'a dyn model_807::ModelAdapter))
    }

    fn model_808_adapter(&self) -> Option<&'a dyn model_808::ModelAdapter> {
        self.model_808_callback_adapter
            .map(|a| a as &'a dyn model_808::ModelAdapter)
            .or(self
                .model_808_stateful_adapter
                .map(|a| a as &'a dyn model_808::ModelAdapter))
    }

    fn model_809_adapter(&self) -> Option<&'a dyn model_809::ModelAdapter> {
        self.model_809_callback_adapter
            .map(|a| a as &'a dyn model_809::ModelAdapter)
            .or(self
                .model_809_stateful_adapter
                .map(|a| a as &'a dyn model_809::ModelAdapter))
    }

    fn model_63001_adapter(&self) -> Option<&'a dyn model_63001::ModelAdapter> {
        self.model_63001_callback_adapter
            .map(|a| a as &'a dyn model_63001::ModelAdapter)
            .or(self
                .model_63001_stateful_adapter
                .map(|a| a as &'a dyn model_63001::ModelAdapter))
    }

    fn model_64001_adapter(&self) -> Option<&'a dyn model_64001::ModelAdapter> {
        self.model_64001_callback_adapter
            .map(|a| a as &'a dyn model_64001::ModelAdapter)
            .or(self
                .model_64001_stateful_adapter
                .map(|a| a as &'a dyn model_64001::ModelAdapter))
    }

    fn model_64020_adapter(&self) -> Option<&'a dyn model_64020::ModelAdapter> {
        self.model_64020_callback_adapter
            .map(|a| a as &'a dyn model_64020::ModelAdapter)
            .or(self
                .model_64020_stateful_adapter
                .map(|a| a as &'a dyn model_64020::ModelAdapter))
    }

    fn model_64101_adapter(&self) -> Option<&'a dyn model_64101::ModelAdapter> {
        self.model_64101_callback_adapter
            .map(|a| a as &'a dyn model_64101::ModelAdapter)
            .or(self
                .model_64101_stateful_adapter
                .map(|a| a as &'a dyn model_64101::ModelAdapter))
    }

    fn model_64111_adapter(&self) -> Option<&'a dyn model_64111::ModelAdapter> {
        self.model_64111_callback_adapter
            .map(|a| a as &'a dyn model_64111::ModelAdapter)
            .or(self
                .model_64111_stateful_adapter
                .map(|a| a as &'a dyn model_64111::ModelAdapter))
    }

    fn model_64112_adapter(&self) -> Option<&'a dyn model_64112::ModelAdapter> {
        self.model_64112_callback_adapter
            .map(|a| a as &'a dyn model_64112::ModelAdapter)
            .or(self
                .model_64112_stateful_adapter
                .map(|a| a as &'a dyn model_64112::ModelAdapter))
    }

    fn model_64410_adapter(&self) -> Option<&'a dyn model_64410::ModelAdapter> {
        self.model_64410_callback_adapter
            .map(|a| a as &'a dyn model_64410::ModelAdapter)
    }

    fn model_64411_adapter(&self) -> Option<&'a dyn model_64411::ModelAdapter> {
        self.model_64411_callback_adapter
            .map(|a| a as &'a dyn model_64411::ModelAdapter)
    }

    fn model_64412_adapter(&self) -> Option<&'a dyn model_64412::ModelAdapter> {
        self.model_64412_callback_adapter
            .map(|a| a as &'a dyn model_64412::ModelAdapter)
            .or(self
                .model_64412_stateful_adapter
                .map(|a| a as &'a dyn model_64412::ModelAdapter))
    }

    fn model_64413_adapter(&self) -> Option<&'a dyn model_64413::ModelAdapter> {
        self.model_64413_callback_adapter
            .map(|a| a as &'a dyn model_64413::ModelAdapter)
    }

    fn model_64414_adapter(&self) -> Option<&'a dyn model_64414::ModelAdapter> {
        self.model_64414_callback_adapter
            .map(|a| a as &'a dyn model_64414::ModelAdapter)
            .or(self
                .model_64414_stateful_adapter
                .map(|a| a as &'a dyn model_64414::ModelAdapter))
    }

    fn model_64415_adapter(&self) -> Option<&'a dyn model_64415::ModelAdapter> {
        self.model_64415_callback_adapter
            .map(|a| a as &'a dyn model_64415::ModelAdapter)
            .or(self
                .model_64415_stateful_adapter
                .map(|a| a as &'a dyn model_64415::ModelAdapter))
    }
}

pub fn read_into_buffer<'a, 'b>(
    adapters: &'a dyn SunspecAdapterProvider<'a>,
    buffer: &'b mut ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) -> Option<()> {
    let mut cursor = Cursor {
        source_offset: offset,
        buffer_offset: 0,
        limit,
    };

    cursor.handle_static_read(2, |offset, from, len| {
        write_string(c"SunS", buffer.slice(from, len), offset, len)
    })?;
    cursor.handle_adapter_read(
        adapters.model_1_adapter(),
        model_1::model_length,
        |adapter, offset, from, len| {
            model_1::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_2_adapter(),
        model_2::model_length,
        |adapter, offset, from, len| {
            model_2::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_3_adapter(),
        model_3::model_length,
        |adapter, offset, from, len| {
            model_3::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_4_adapter(),
        model_4::model_length,
        |adapter, offset, from, len| {
            model_4::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_5_adapter(),
        model_5::model_length,
        |adapter, offset, from, len| {
            model_5::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_6_adapter(),
        model_6::model_length,
        |adapter, offset, from, len| {
            model_6::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_7_adapter(),
        model_7::model_length,
        |adapter, offset, from, len| {
            model_7::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_8_adapter(),
        model_8::model_length,
        |adapter, offset, from, len| {
            model_8::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_10_adapter(),
        model_10::model_length,
        |adapter, offset, from, len| {
            model_10::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_11_adapter(),
        model_11::model_length,
        |adapter, offset, from, len| {
            model_11::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_12_adapter(),
        model_12::model_length,
        |adapter, offset, from, len| {
            model_12::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_13_adapter(),
        model_13::model_length,
        |adapter, offset, from, len| {
            model_13::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_15_adapter(),
        model_15::model_length,
        |adapter, offset, from, len| {
            model_15::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_16_adapter(),
        model_16::model_length,
        |adapter, offset, from, len| {
            model_16::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_17_adapter(),
        model_17::model_length,
        |adapter, offset, from, len| {
            model_17::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_18_adapter(),
        model_18::model_length,
        |adapter, offset, from, len| {
            model_18::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_19_adapter(),
        model_19::model_length,
        |adapter, offset, from, len| {
            model_19::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_101_adapter(),
        model_101::model_length,
        |adapter, offset, from, len| {
            model_101::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_102_adapter(),
        model_102::model_length,
        |adapter, offset, from, len| {
            model_102::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_103_adapter(),
        model_103::model_length,
        |adapter, offset, from, len| {
            model_103::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_111_adapter(),
        model_111::model_length,
        |adapter, offset, from, len| {
            model_111::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_112_adapter(),
        model_112::model_length,
        |adapter, offset, from, len| {
            model_112::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_113_adapter(),
        model_113::model_length,
        |adapter, offset, from, len| {
            model_113::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_120_adapter(),
        model_120::model_length,
        |adapter, offset, from, len| {
            model_120::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_121_adapter(),
        model_121::model_length,
        |adapter, offset, from, len| {
            model_121::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_122_adapter(),
        model_122::model_length,
        |adapter, offset, from, len| {
            model_122::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_123_adapter(),
        model_123::model_length,
        |adapter, offset, from, len| {
            model_123::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_124_adapter(),
        model_124::model_length,
        |adapter, offset, from, len| {
            model_124::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_125_adapter(),
        model_125::model_length,
        |adapter, offset, from, len| {
            model_125::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_126_adapter(),
        model_126::model_length,
        |adapter, offset, from, len| {
            model_126::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_127_adapter(),
        model_127::model_length,
        |adapter, offset, from, len| {
            model_127::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_128_adapter(),
        model_128::model_length,
        |adapter, offset, from, len| {
            model_128::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_129_adapter(),
        model_129::model_length,
        |adapter, offset, from, len| {
            model_129::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_130_adapter(),
        model_130::model_length,
        |adapter, offset, from, len| {
            model_130::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_131_adapter(),
        model_131::model_length,
        |adapter, offset, from, len| {
            model_131::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_132_adapter(),
        model_132::model_length,
        |adapter, offset, from, len| {
            model_132::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_133_adapter(),
        model_133::model_length,
        |adapter, offset, from, len| {
            model_133::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_134_adapter(),
        model_134::model_length,
        |adapter, offset, from, len| {
            model_134::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_135_adapter(),
        model_135::model_length,
        |adapter, offset, from, len| {
            model_135::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_136_adapter(),
        model_136::model_length,
        |adapter, offset, from, len| {
            model_136::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_137_adapter(),
        model_137::model_length,
        |adapter, offset, from, len| {
            model_137::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_138_adapter(),
        model_138::model_length,
        |adapter, offset, from, len| {
            model_138::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_139_adapter(),
        model_139::model_length,
        |adapter, offset, from, len| {
            model_139::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_140_adapter(),
        model_140::model_length,
        |adapter, offset, from, len| {
            model_140::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_141_adapter(),
        model_141::model_length,
        |adapter, offset, from, len| {
            model_141::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_142_adapter(),
        model_142::model_length,
        |adapter, offset, from, len| {
            model_142::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_143_adapter(),
        model_143::model_length,
        |adapter, offset, from, len| {
            model_143::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_144_adapter(),
        model_144::model_length,
        |adapter, offset, from, len| {
            model_144::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_145_adapter(),
        model_145::model_length,
        |adapter, offset, from, len| {
            model_145::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_160_adapter(),
        model_160::model_length,
        |adapter, offset, from, len| {
            model_160::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_201_adapter(),
        model_201::model_length,
        |adapter, offset, from, len| {
            model_201::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_202_adapter(),
        model_202::model_length,
        |adapter, offset, from, len| {
            model_202::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_203_adapter(),
        model_203::model_length,
        |adapter, offset, from, len| {
            model_203::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_204_adapter(),
        model_204::model_length,
        |adapter, offset, from, len| {
            model_204::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_211_adapter(),
        model_211::model_length,
        |adapter, offset, from, len| {
            model_211::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_212_adapter(),
        model_212::model_length,
        |adapter, offset, from, len| {
            model_212::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_213_adapter(),
        model_213::model_length,
        |adapter, offset, from, len| {
            model_213::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_214_adapter(),
        model_214::model_length,
        |adapter, offset, from, len| {
            model_214::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_220_adapter(),
        model_220::model_length,
        |adapter, offset, from, len| {
            model_220::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_305_adapter(),
        model_305::model_length,
        |adapter, offset, from, len| {
            model_305::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_306_adapter(),
        model_306::model_length,
        |adapter, offset, from, len| {
            model_306::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_307_adapter(),
        model_307::model_length,
        |adapter, offset, from, len| {
            model_307::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_308_adapter(),
        model_308::model_length,
        |adapter, offset, from, len| {
            model_308::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_401_adapter(),
        model_401::model_length,
        |adapter, offset, from, len| {
            model_401::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_402_adapter(),
        model_402::model_length,
        |adapter, offset, from, len| {
            model_402::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_403_adapter(),
        model_403::model_length,
        |adapter, offset, from, len| {
            model_403::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_404_adapter(),
        model_404::model_length,
        |adapter, offset, from, len| {
            model_404::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_501_adapter(),
        model_501::model_length,
        |adapter, offset, from, len| {
            model_501::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_502_adapter(),
        model_502::model_length,
        |adapter, offset, from, len| {
            model_502::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_701_adapter(),
        model_701::model_length,
        |adapter, offset, from, len| {
            model_701::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_703_adapter(),
        model_703::model_length,
        |adapter, offset, from, len| {
            model_703::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_704_adapter(),
        model_704::model_length,
        |adapter, offset, from, len| {
            model_704::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_705_adapter(),
        model_705::model_length,
        |adapter, offset, from, len| {
            model_705::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_706_adapter(),
        model_706::model_length,
        |adapter, offset, from, len| {
            model_706::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_707_adapter(),
        model_707::model_length,
        |adapter, offset, from, len| {
            model_707::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_708_adapter(),
        model_708::model_length,
        |adapter, offset, from, len| {
            model_708::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_709_adapter(),
        model_709::model_length,
        |adapter, offset, from, len| {
            model_709::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_710_adapter(),
        model_710::model_length,
        |adapter, offset, from, len| {
            model_710::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_711_adapter(),
        model_711::model_length,
        |adapter, offset, from, len| {
            model_711::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_712_adapter(),
        model_712::model_length,
        |adapter, offset, from, len| {
            model_712::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_713_adapter(),
        model_713::model_length,
        |adapter, offset, from, len| {
            model_713::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_714_adapter(),
        model_714::model_length,
        |adapter, offset, from, len| {
            model_714::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_715_adapter(),
        model_715::model_length,
        |adapter, offset, from, len| {
            model_715::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_801_adapter(),
        model_801::model_length,
        |adapter, offset, from, len| {
            model_801::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_802_adapter(),
        model_802::model_length,
        |adapter, offset, from, len| {
            model_802::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_805_adapter(),
        model_805::model_length,
        |adapter, offset, from, len| {
            model_805::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_806_adapter(),
        model_806::model_length,
        |adapter, offset, from, len| {
            model_806::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_807_adapter(),
        model_807::model_length,
        |adapter, offset, from, len| {
            model_807::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_808_adapter(),
        model_808::model_length,
        |adapter, offset, from, len| {
            model_808::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_809_adapter(),
        model_809::model_length,
        |adapter, offset, from, len| {
            model_809::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_63001_adapter(),
        model_63001::model_length,
        |adapter, offset, from, len| {
            model_63001::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64001_adapter(),
        model_64001::model_length,
        |adapter, offset, from, len| {
            model_64001::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64020_adapter(),
        model_64020::model_length,
        |adapter, offset, from, len| {
            model_64020::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64101_adapter(),
        model_64101::model_length,
        |adapter, offset, from, len| {
            model_64101::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64111_adapter(),
        model_64111::model_length,
        |adapter, offset, from, len| {
            model_64111::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64112_adapter(),
        model_64112::model_length,
        |adapter, offset, from, len| {
            model_64112::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64410_adapter(),
        model_64410::model_length,
        |adapter, offset, from, len| {
            model_64410::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64411_adapter(),
        model_64411::model_length,
        |adapter, offset, from, len| {
            model_64411::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64412_adapter(),
        model_64412::model_length,
        |adapter, offset, from, len| {
            model_64412::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64413_adapter(),
        model_64413::model_length,
        |adapter, offset, from, len| {
            model_64413::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64414_adapter(),
        model_64414::model_length,
        |adapter, offset, from, len| {
            model_64414::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_adapter_read(
        adapters.model_64415_adapter(),
        model_64415::model_length,
        |adapter, offset, from, len| {
            model_64415::read_into_buffer(adapter, &mut buffer.slice(from, len), offset, len)
        },
    )?;
    cursor.handle_static_read(1, |_, from, len| write_u16(0xffff, buffer.slice(from, len)))?;
    Some(())
}
