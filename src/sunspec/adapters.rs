use crate::sunspec::models::{model_1, model_2, model_3, model_4, model_5, model_6, model_7, model_8, model_10, model_11, model_12, model_13, model_15, model_16, model_17, model_18, model_19, model_101, model_102, model_103, model_111, model_112, model_113, model_120, model_121, model_122, model_123, model_124, model_125, model_126, model_127, model_128, model_129, model_130, model_131, model_132, model_133, model_134, model_135, model_136, model_137, model_138, model_139, model_140, model_141, model_142, model_143, model_144, model_145, model_160, model_201, model_202, model_203, model_204, model_211, model_212, model_213, model_214, model_220, model_305, model_306, model_307, model_308, model_401, model_402, model_403, model_404, model_501, model_502, model_701, model_703, model_704, model_705, model_706, model_707, model_708, model_709, model_710, model_711, model_712, model_713, model_714, model_715, model_801, model_802, model_803, model_804, model_805, model_806, model_807, model_808, model_809, model_63001, model_64001, model_64020, model_64101, model_64111, model_64112, model_64410, model_64411, model_64412, model_64413, model_64414, model_64415};
use crate::ReadablePoint;
use crate::sunspec::points::PointReference;

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

    fn model_803_adapter(&self) -> Option<&'a dyn model_803::ModelAdapter>;

    fn model_804_adapter(&self) -> Option<&'a dyn model_804::ModelAdapter>;

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
    pub model_803_adapter: Option<&'a dyn model_803::ModelAdapter>,
    pub model_804_adapter: Option<&'a dyn model_804::ModelAdapter>,
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

    fn model_803_adapter(&self) -> Option<&'a dyn model_803::ModelAdapter> {
        self.model_803_adapter
    }

    fn model_804_adapter(&self) -> Option<&'a dyn model_804::ModelAdapter> {
        self.model_804_adapter
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
    pub model_705_stateful_adapter: Option<&'a model_705::Model705StatefulAdapter>,
    pub model_706_callback_adapter: Option<&'a model_706::Model706CallbackAdapter>,
    pub model_706_stateful_adapter: Option<&'a model_706::Model706StatefulAdapter>,
    pub model_707_callback_adapter: Option<&'a model_707::Model707CallbackAdapter>,
    pub model_707_stateful_adapter: Option<&'a model_707::Model707StatefulAdapter>,
    pub model_708_callback_adapter: Option<&'a model_708::Model708CallbackAdapter>,
    pub model_708_stateful_adapter: Option<&'a model_708::Model708StatefulAdapter>,
    pub model_709_callback_adapter: Option<&'a model_709::Model709CallbackAdapter>,
    pub model_709_stateful_adapter: Option<&'a model_709::Model709StatefulAdapter>,
    pub model_710_callback_adapter: Option<&'a model_710::Model710CallbackAdapter>,
    pub model_710_stateful_adapter: Option<&'a model_710::Model710StatefulAdapter>,
    pub model_711_callback_adapter: Option<&'a model_711::Model711CallbackAdapter>,
    pub model_711_stateful_adapter: Option<&'a model_711::Model711StatefulAdapter>,
    pub model_712_callback_adapter: Option<&'a model_712::Model712CallbackAdapter>,
    pub model_712_stateful_adapter: Option<&'a model_712::Model712StatefulAdapter>,
    pub model_713_callback_adapter: Option<&'a model_713::Model713CallbackAdapter>,
    pub model_713_stateful_adapter: Option<&'a model_713::Model713StatefulAdapter>,
    pub model_714_callback_adapter: Option<&'a model_714::Model714CallbackAdapter>,
    pub model_714_stateful_adapter: Option<&'a model_714::Model714StatefulAdapter>,
    pub model_715_callback_adapter: Option<&'a model_715::Model715CallbackAdapter>,
    pub model_715_stateful_adapter: Option<&'a model_715::Model715StatefulAdapter>,
    pub model_801_callback_adapter: Option<&'a model_801::Model801CallbackAdapter>,
    pub model_801_stateful_adapter: Option<&'a model_801::Model801StatefulAdapter>,
    pub model_802_callback_adapter: Option<&'a model_802::Model802CallbackAdapter>,
    pub model_802_stateful_adapter: Option<&'a model_802::Model802StatefulAdapter>,
    pub model_803_callback_adapter: Option<&'a model_803::Model803CallbackAdapter>,
    pub model_803_stateful_adapter: Option<&'a model_803::Model803StatefulAdapter>,
    pub model_804_callback_adapter: Option<&'a model_804::Model804CallbackAdapter>,
    pub model_804_stateful_adapter: Option<&'a model_804::Model804StatefulAdapter>,
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
    pub model_64410_stateful_adapter: Option<&'a model_64410::Model64410StatefulAdapter>,
    pub model_64411_callback_adapter: Option<&'a model_64411::Model64411CallbackAdapter>,
    pub model_64411_stateful_adapter: Option<&'a model_64411::Model64411StatefulAdapter>,
    pub model_64412_callback_adapter: Option<&'a model_64412::Model64412CallbackAdapter>,
    pub model_64412_stateful_adapter: Option<&'a model_64412::Model64412StatefulAdapter>,
    pub model_64413_callback_adapter: Option<&'a model_64413::Model64413CallbackAdapter>,
    pub model_64413_stateful_adapter: Option<&'a model_64413::Model64413StatefulAdapter>,
    pub model_64414_callback_adapter: Option<&'a model_64414::Model64414CallbackAdapter>,
    pub model_64414_stateful_adapter: Option<&'a model_64414::Model64414StatefulAdapter>,
    pub model_64415_callback_adapter: Option<&'a model_64415::Model64415CallbackAdapter>,
    pub model_64415_stateful_adapter: Option<&'a model_64415::Model64415StatefulAdapter>,
}

impl<'a> SunspecAdapterProvider<'a> for SunspecExternalAdapters<'a> {
    fn model_1_adapter(&self) -> Option<&'a dyn model_1::ModelAdapter> {
        self.model_1_callback_adapter
        .map(|a| a as &'a dyn model_1::ModelAdapter)
        .or(self.model_1_stateful_adapter.map(|a| a as &'a dyn model_1::ModelAdapter))
    }

    fn model_2_adapter(&self) -> Option<&'a dyn model_2::ModelAdapter> {
        self.model_2_callback_adapter
        .map(|a| a as &'a dyn model_2::ModelAdapter)
        .or(self.model_2_stateful_adapter.map(|a| a as &'a dyn model_2::ModelAdapter))
    }

    fn model_3_adapter(&self) -> Option<&'a dyn model_3::ModelAdapter> {
        self.model_3_callback_adapter
        .map(|a| a as &'a dyn model_3::ModelAdapter)
        .or(self.model_3_stateful_adapter.map(|a| a as &'a dyn model_3::ModelAdapter))
    }

    fn model_4_adapter(&self) -> Option<&'a dyn model_4::ModelAdapter> {
        self.model_4_callback_adapter
        .map(|a| a as &'a dyn model_4::ModelAdapter)
        .or(self.model_4_stateful_adapter.map(|a| a as &'a dyn model_4::ModelAdapter))
    }

    fn model_5_adapter(&self) -> Option<&'a dyn model_5::ModelAdapter> {
        self.model_5_callback_adapter
        .map(|a| a as &'a dyn model_5::ModelAdapter)
        .or(self.model_5_stateful_adapter.map(|a| a as &'a dyn model_5::ModelAdapter))
    }

    fn model_6_adapter(&self) -> Option<&'a dyn model_6::ModelAdapter> {
        self.model_6_callback_adapter
        .map(|a| a as &'a dyn model_6::ModelAdapter)
        .or(self.model_6_stateful_adapter.map(|a| a as &'a dyn model_6::ModelAdapter))
    }

    fn model_7_adapter(&self) -> Option<&'a dyn model_7::ModelAdapter> {
        self.model_7_callback_adapter
        .map(|a| a as &'a dyn model_7::ModelAdapter)
        .or(self.model_7_stateful_adapter.map(|a| a as &'a dyn model_7::ModelAdapter))
    }

    fn model_8_adapter(&self) -> Option<&'a dyn model_8::ModelAdapter> {
        self.model_8_callback_adapter
        .map(|a| a as &'a dyn model_8::ModelAdapter)
        .or(self.model_8_stateful_adapter.map(|a| a as &'a dyn model_8::ModelAdapter))
    }

    fn model_10_adapter(&self) -> Option<&'a dyn model_10::ModelAdapter> {
        self.model_10_callback_adapter
        .map(|a| a as &'a dyn model_10::ModelAdapter)
        .or(self.model_10_stateful_adapter.map(|a| a as &'a dyn model_10::ModelAdapter))
    }

    fn model_11_adapter(&self) -> Option<&'a dyn model_11::ModelAdapter> {
        self.model_11_callback_adapter
        .map(|a| a as &'a dyn model_11::ModelAdapter)
        .or(self.model_11_stateful_adapter.map(|a| a as &'a dyn model_11::ModelAdapter))
    }

    fn model_12_adapter(&self) -> Option<&'a dyn model_12::ModelAdapter> {
        self.model_12_callback_adapter
        .map(|a| a as &'a dyn model_12::ModelAdapter)
        .or(self.model_12_stateful_adapter.map(|a| a as &'a dyn model_12::ModelAdapter))
    }

    fn model_13_adapter(&self) -> Option<&'a dyn model_13::ModelAdapter> {
        self.model_13_callback_adapter
        .map(|a| a as &'a dyn model_13::ModelAdapter)
        .or(self.model_13_stateful_adapter.map(|a| a as &'a dyn model_13::ModelAdapter))
    }

    fn model_15_adapter(&self) -> Option<&'a dyn model_15::ModelAdapter> {
        self.model_15_callback_adapter
        .map(|a| a as &'a dyn model_15::ModelAdapter)
        .or(self.model_15_stateful_adapter.map(|a| a as &'a dyn model_15::ModelAdapter))
    }

    fn model_16_adapter(&self) -> Option<&'a dyn model_16::ModelAdapter> {
        self.model_16_callback_adapter
        .map(|a| a as &'a dyn model_16::ModelAdapter)
        .or(self.model_16_stateful_adapter.map(|a| a as &'a dyn model_16::ModelAdapter))
    }

    fn model_17_adapter(&self) -> Option<&'a dyn model_17::ModelAdapter> {
        self.model_17_callback_adapter
        .map(|a| a as &'a dyn model_17::ModelAdapter)
        .or(self.model_17_stateful_adapter.map(|a| a as &'a dyn model_17::ModelAdapter))
    }

    fn model_18_adapter(&self) -> Option<&'a dyn model_18::ModelAdapter> {
        self.model_18_callback_adapter
        .map(|a| a as &'a dyn model_18::ModelAdapter)
        .or(self.model_18_stateful_adapter.map(|a| a as &'a dyn model_18::ModelAdapter))
    }

    fn model_19_adapter(&self) -> Option<&'a dyn model_19::ModelAdapter> {
        self.model_19_callback_adapter
        .map(|a| a as &'a dyn model_19::ModelAdapter)
        .or(self.model_19_stateful_adapter.map(|a| a as &'a dyn model_19::ModelAdapter))
    }

    fn model_101_adapter(&self) -> Option<&'a dyn model_101::ModelAdapter> {
        self.model_101_callback_adapter
        .map(|a| a as &'a dyn model_101::ModelAdapter)
        .or(self.model_101_stateful_adapter.map(|a| a as &'a dyn model_101::ModelAdapter))
    }

    fn model_102_adapter(&self) -> Option<&'a dyn model_102::ModelAdapter> {
        self.model_102_callback_adapter
        .map(|a| a as &'a dyn model_102::ModelAdapter)
        .or(self.model_102_stateful_adapter.map(|a| a as &'a dyn model_102::ModelAdapter))
    }

    fn model_103_adapter(&self) -> Option<&'a dyn model_103::ModelAdapter> {
        self.model_103_callback_adapter
        .map(|a| a as &'a dyn model_103::ModelAdapter)
        .or(self.model_103_stateful_adapter.map(|a| a as &'a dyn model_103::ModelAdapter))
    }

    fn model_111_adapter(&self) -> Option<&'a dyn model_111::ModelAdapter> {
        self.model_111_callback_adapter
        .map(|a| a as &'a dyn model_111::ModelAdapter)
        .or(self.model_111_stateful_adapter.map(|a| a as &'a dyn model_111::ModelAdapter))
    }

    fn model_112_adapter(&self) -> Option<&'a dyn model_112::ModelAdapter> {
        self.model_112_callback_adapter
        .map(|a| a as &'a dyn model_112::ModelAdapter)
        .or(self.model_112_stateful_adapter.map(|a| a as &'a dyn model_112::ModelAdapter))
    }

    fn model_113_adapter(&self) -> Option<&'a dyn model_113::ModelAdapter> {
        self.model_113_callback_adapter
        .map(|a| a as &'a dyn model_113::ModelAdapter)
        .or(self.model_113_stateful_adapter.map(|a| a as &'a dyn model_113::ModelAdapter))
    }

    fn model_120_adapter(&self) -> Option<&'a dyn model_120::ModelAdapter> {
        self.model_120_callback_adapter
        .map(|a| a as &'a dyn model_120::ModelAdapter)
        .or(self.model_120_stateful_adapter.map(|a| a as &'a dyn model_120::ModelAdapter))
    }

    fn model_121_adapter(&self) -> Option<&'a dyn model_121::ModelAdapter> {
        self.model_121_callback_adapter
        .map(|a| a as &'a dyn model_121::ModelAdapter)
        .or(self.model_121_stateful_adapter.map(|a| a as &'a dyn model_121::ModelAdapter))
    }

    fn model_122_adapter(&self) -> Option<&'a dyn model_122::ModelAdapter> {
        self.model_122_callback_adapter
        .map(|a| a as &'a dyn model_122::ModelAdapter)
        .or(self.model_122_stateful_adapter.map(|a| a as &'a dyn model_122::ModelAdapter))
    }

    fn model_123_adapter(&self) -> Option<&'a dyn model_123::ModelAdapter> {
        self.model_123_callback_adapter
        .map(|a| a as &'a dyn model_123::ModelAdapter)
        .or(self.model_123_stateful_adapter.map(|a| a as &'a dyn model_123::ModelAdapter))
    }

    fn model_124_adapter(&self) -> Option<&'a dyn model_124::ModelAdapter> {
        self.model_124_callback_adapter
        .map(|a| a as &'a dyn model_124::ModelAdapter)
        .or(self.model_124_stateful_adapter.map(|a| a as &'a dyn model_124::ModelAdapter))
    }

    fn model_125_adapter(&self) -> Option<&'a dyn model_125::ModelAdapter> {
        self.model_125_callback_adapter
        .map(|a| a as &'a dyn model_125::ModelAdapter)
        .or(self.model_125_stateful_adapter.map(|a| a as &'a dyn model_125::ModelAdapter))
    }

    fn model_126_adapter(&self) -> Option<&'a dyn model_126::ModelAdapter> {
        self.model_126_callback_adapter
        .map(|a| a as &'a dyn model_126::ModelAdapter)
        .or(self.model_126_stateful_adapter.map(|a| a as &'a dyn model_126::ModelAdapter))
    }

    fn model_127_adapter(&self) -> Option<&'a dyn model_127::ModelAdapter> {
        self.model_127_callback_adapter
        .map(|a| a as &'a dyn model_127::ModelAdapter)
        .or(self.model_127_stateful_adapter.map(|a| a as &'a dyn model_127::ModelAdapter))
    }

    fn model_128_adapter(&self) -> Option<&'a dyn model_128::ModelAdapter> {
        self.model_128_callback_adapter
        .map(|a| a as &'a dyn model_128::ModelAdapter)
        .or(self.model_128_stateful_adapter.map(|a| a as &'a dyn model_128::ModelAdapter))
    }

    fn model_129_adapter(&self) -> Option<&'a dyn model_129::ModelAdapter> {
        self.model_129_callback_adapter
        .map(|a| a as &'a dyn model_129::ModelAdapter)
        .or(self.model_129_stateful_adapter.map(|a| a as &'a dyn model_129::ModelAdapter))
    }

    fn model_130_adapter(&self) -> Option<&'a dyn model_130::ModelAdapter> {
        self.model_130_callback_adapter
        .map(|a| a as &'a dyn model_130::ModelAdapter)
        .or(self.model_130_stateful_adapter.map(|a| a as &'a dyn model_130::ModelAdapter))
    }

    fn model_131_adapter(&self) -> Option<&'a dyn model_131::ModelAdapter> {
        self.model_131_callback_adapter
        .map(|a| a as &'a dyn model_131::ModelAdapter)
        .or(self.model_131_stateful_adapter.map(|a| a as &'a dyn model_131::ModelAdapter))
    }

    fn model_132_adapter(&self) -> Option<&'a dyn model_132::ModelAdapter> {
        self.model_132_callback_adapter
        .map(|a| a as &'a dyn model_132::ModelAdapter)
        .or(self.model_132_stateful_adapter.map(|a| a as &'a dyn model_132::ModelAdapter))
    }

    fn model_133_adapter(&self) -> Option<&'a dyn model_133::ModelAdapter> {
        self.model_133_callback_adapter
        .map(|a| a as &'a dyn model_133::ModelAdapter)
        .or(self.model_133_stateful_adapter.map(|a| a as &'a dyn model_133::ModelAdapter))
    }

    fn model_134_adapter(&self) -> Option<&'a dyn model_134::ModelAdapter> {
        self.model_134_callback_adapter
        .map(|a| a as &'a dyn model_134::ModelAdapter)
        .or(self.model_134_stateful_adapter.map(|a| a as &'a dyn model_134::ModelAdapter))
    }

    fn model_135_adapter(&self) -> Option<&'a dyn model_135::ModelAdapter> {
        self.model_135_callback_adapter
        .map(|a| a as &'a dyn model_135::ModelAdapter)
        .or(self.model_135_stateful_adapter.map(|a| a as &'a dyn model_135::ModelAdapter))
    }

    fn model_136_adapter(&self) -> Option<&'a dyn model_136::ModelAdapter> {
        self.model_136_callback_adapter
        .map(|a| a as &'a dyn model_136::ModelAdapter)
        .or(self.model_136_stateful_adapter.map(|a| a as &'a dyn model_136::ModelAdapter))
    }

    fn model_137_adapter(&self) -> Option<&'a dyn model_137::ModelAdapter> {
        self.model_137_callback_adapter
        .map(|a| a as &'a dyn model_137::ModelAdapter)
        .or(self.model_137_stateful_adapter.map(|a| a as &'a dyn model_137::ModelAdapter))
    }

    fn model_138_adapter(&self) -> Option<&'a dyn model_138::ModelAdapter> {
        self.model_138_callback_adapter
        .map(|a| a as &'a dyn model_138::ModelAdapter)
        .or(self.model_138_stateful_adapter.map(|a| a as &'a dyn model_138::ModelAdapter))
    }

    fn model_139_adapter(&self) -> Option<&'a dyn model_139::ModelAdapter> {
        self.model_139_callback_adapter
        .map(|a| a as &'a dyn model_139::ModelAdapter)
        .or(self.model_139_stateful_adapter.map(|a| a as &'a dyn model_139::ModelAdapter))
    }

    fn model_140_adapter(&self) -> Option<&'a dyn model_140::ModelAdapter> {
        self.model_140_callback_adapter
        .map(|a| a as &'a dyn model_140::ModelAdapter)
        .or(self.model_140_stateful_adapter.map(|a| a as &'a dyn model_140::ModelAdapter))
    }

    fn model_141_adapter(&self) -> Option<&'a dyn model_141::ModelAdapter> {
        self.model_141_callback_adapter
        .map(|a| a as &'a dyn model_141::ModelAdapter)
        .or(self.model_141_stateful_adapter.map(|a| a as &'a dyn model_141::ModelAdapter))
    }

    fn model_142_adapter(&self) -> Option<&'a dyn model_142::ModelAdapter> {
        self.model_142_callback_adapter
        .map(|a| a as &'a dyn model_142::ModelAdapter)
        .or(self.model_142_stateful_adapter.map(|a| a as &'a dyn model_142::ModelAdapter))
    }

    fn model_143_adapter(&self) -> Option<&'a dyn model_143::ModelAdapter> {
        self.model_143_callback_adapter
        .map(|a| a as &'a dyn model_143::ModelAdapter)
        .or(self.model_143_stateful_adapter.map(|a| a as &'a dyn model_143::ModelAdapter))
    }

    fn model_144_adapter(&self) -> Option<&'a dyn model_144::ModelAdapter> {
        self.model_144_callback_adapter
        .map(|a| a as &'a dyn model_144::ModelAdapter)
        .or(self.model_144_stateful_adapter.map(|a| a as &'a dyn model_144::ModelAdapter))
    }

    fn model_145_adapter(&self) -> Option<&'a dyn model_145::ModelAdapter> {
        self.model_145_callback_adapter
        .map(|a| a as &'a dyn model_145::ModelAdapter)
        .or(self.model_145_stateful_adapter.map(|a| a as &'a dyn model_145::ModelAdapter))
    }

    fn model_160_adapter(&self) -> Option<&'a dyn model_160::ModelAdapter> {
        self.model_160_callback_adapter
        .map(|a| a as &'a dyn model_160::ModelAdapter)
        .or(self.model_160_stateful_adapter.map(|a| a as &'a dyn model_160::ModelAdapter))
    }

    fn model_201_adapter(&self) -> Option<&'a dyn model_201::ModelAdapter> {
        self.model_201_callback_adapter
        .map(|a| a as &'a dyn model_201::ModelAdapter)
        .or(self.model_201_stateful_adapter.map(|a| a as &'a dyn model_201::ModelAdapter))
    }

    fn model_202_adapter(&self) -> Option<&'a dyn model_202::ModelAdapter> {
        self.model_202_callback_adapter
        .map(|a| a as &'a dyn model_202::ModelAdapter)
        .or(self.model_202_stateful_adapter.map(|a| a as &'a dyn model_202::ModelAdapter))
    }

    fn model_203_adapter(&self) -> Option<&'a dyn model_203::ModelAdapter> {
        self.model_203_callback_adapter
        .map(|a| a as &'a dyn model_203::ModelAdapter)
        .or(self.model_203_stateful_adapter.map(|a| a as &'a dyn model_203::ModelAdapter))
    }

    fn model_204_adapter(&self) -> Option<&'a dyn model_204::ModelAdapter> {
        self.model_204_callback_adapter
        .map(|a| a as &'a dyn model_204::ModelAdapter)
        .or(self.model_204_stateful_adapter.map(|a| a as &'a dyn model_204::ModelAdapter))
    }

    fn model_211_adapter(&self) -> Option<&'a dyn model_211::ModelAdapter> {
        self.model_211_callback_adapter
        .map(|a| a as &'a dyn model_211::ModelAdapter)
        .or(self.model_211_stateful_adapter.map(|a| a as &'a dyn model_211::ModelAdapter))
    }

    fn model_212_adapter(&self) -> Option<&'a dyn model_212::ModelAdapter> {
        self.model_212_callback_adapter
        .map(|a| a as &'a dyn model_212::ModelAdapter)
        .or(self.model_212_stateful_adapter.map(|a| a as &'a dyn model_212::ModelAdapter))
    }

    fn model_213_adapter(&self) -> Option<&'a dyn model_213::ModelAdapter> {
        self.model_213_callback_adapter
        .map(|a| a as &'a dyn model_213::ModelAdapter)
        .or(self.model_213_stateful_adapter.map(|a| a as &'a dyn model_213::ModelAdapter))
    }

    fn model_214_adapter(&self) -> Option<&'a dyn model_214::ModelAdapter> {
        self.model_214_callback_adapter
        .map(|a| a as &'a dyn model_214::ModelAdapter)
        .or(self.model_214_stateful_adapter.map(|a| a as &'a dyn model_214::ModelAdapter))
    }

    fn model_220_adapter(&self) -> Option<&'a dyn model_220::ModelAdapter> {
        self.model_220_callback_adapter
        .map(|a| a as &'a dyn model_220::ModelAdapter)
        .or(self.model_220_stateful_adapter.map(|a| a as &'a dyn model_220::ModelAdapter))
    }

    fn model_305_adapter(&self) -> Option<&'a dyn model_305::ModelAdapter> {
        self.model_305_callback_adapter
        .map(|a| a as &'a dyn model_305::ModelAdapter)
        .or(self.model_305_stateful_adapter.map(|a| a as &'a dyn model_305::ModelAdapter))
    }

    fn model_306_adapter(&self) -> Option<&'a dyn model_306::ModelAdapter> {
        self.model_306_callback_adapter
        .map(|a| a as &'a dyn model_306::ModelAdapter)
        .or(self.model_306_stateful_adapter.map(|a| a as &'a dyn model_306::ModelAdapter))
    }

    fn model_307_adapter(&self) -> Option<&'a dyn model_307::ModelAdapter> {
        self.model_307_callback_adapter
        .map(|a| a as &'a dyn model_307::ModelAdapter)
        .or(self.model_307_stateful_adapter.map(|a| a as &'a dyn model_307::ModelAdapter))
    }

    fn model_308_adapter(&self) -> Option<&'a dyn model_308::ModelAdapter> {
        self.model_308_callback_adapter
        .map(|a| a as &'a dyn model_308::ModelAdapter)
        .or(self.model_308_stateful_adapter.map(|a| a as &'a dyn model_308::ModelAdapter))
    }

    fn model_401_adapter(&self) -> Option<&'a dyn model_401::ModelAdapter> {
        self.model_401_callback_adapter
        .map(|a| a as &'a dyn model_401::ModelAdapter)
        .or(self.model_401_stateful_adapter.map(|a| a as &'a dyn model_401::ModelAdapter))
    }

    fn model_402_adapter(&self) -> Option<&'a dyn model_402::ModelAdapter> {
        self.model_402_callback_adapter
        .map(|a| a as &'a dyn model_402::ModelAdapter)
        .or(self.model_402_stateful_adapter.map(|a| a as &'a dyn model_402::ModelAdapter))
    }

    fn model_403_adapter(&self) -> Option<&'a dyn model_403::ModelAdapter> {
        self.model_403_callback_adapter
        .map(|a| a as &'a dyn model_403::ModelAdapter)
        .or(self.model_403_stateful_adapter.map(|a| a as &'a dyn model_403::ModelAdapter))
    }

    fn model_404_adapter(&self) -> Option<&'a dyn model_404::ModelAdapter> {
        self.model_404_callback_adapter
        .map(|a| a as &'a dyn model_404::ModelAdapter)
        .or(self.model_404_stateful_adapter.map(|a| a as &'a dyn model_404::ModelAdapter))
    }

    fn model_501_adapter(&self) -> Option<&'a dyn model_501::ModelAdapter> {
        self.model_501_callback_adapter
        .map(|a| a as &'a dyn model_501::ModelAdapter)
        .or(self.model_501_stateful_adapter.map(|a| a as &'a dyn model_501::ModelAdapter))
    }

    fn model_502_adapter(&self) -> Option<&'a dyn model_502::ModelAdapter> {
        self.model_502_callback_adapter
        .map(|a| a as &'a dyn model_502::ModelAdapter)
        .or(self.model_502_stateful_adapter.map(|a| a as &'a dyn model_502::ModelAdapter))
    }

    fn model_701_adapter(&self) -> Option<&'a dyn model_701::ModelAdapter> {
        self.model_701_callback_adapter
        .map(|a| a as &'a dyn model_701::ModelAdapter)
        .or(self.model_701_stateful_adapter.map(|a| a as &'a dyn model_701::ModelAdapter))
    }

    fn model_703_adapter(&self) -> Option<&'a dyn model_703::ModelAdapter> {
        self.model_703_callback_adapter
        .map(|a| a as &'a dyn model_703::ModelAdapter)
        .or(self.model_703_stateful_adapter.map(|a| a as &'a dyn model_703::ModelAdapter))
    }

    fn model_704_adapter(&self) -> Option<&'a dyn model_704::ModelAdapter> {
        self.model_704_callback_adapter
        .map(|a| a as &'a dyn model_704::ModelAdapter)
        .or(self.model_704_stateful_adapter.map(|a| a as &'a dyn model_704::ModelAdapter))
    }

    fn model_705_adapter(&self) -> Option<&'a dyn model_705::ModelAdapter> {
        self.model_705_callback_adapter
        .map(|a| a as &'a dyn model_705::ModelAdapter)
        .or(self.model_705_stateful_adapter.map(|a| a as &'a dyn model_705::ModelAdapter))
    }

    fn model_706_adapter(&self) -> Option<&'a dyn model_706::ModelAdapter> {
        self.model_706_callback_adapter
        .map(|a| a as &'a dyn model_706::ModelAdapter)
        .or(self.model_706_stateful_adapter.map(|a| a as &'a dyn model_706::ModelAdapter))
    }

    fn model_707_adapter(&self) -> Option<&'a dyn model_707::ModelAdapter> {
        self.model_707_callback_adapter
        .map(|a| a as &'a dyn model_707::ModelAdapter)
        .or(self.model_707_stateful_adapter.map(|a| a as &'a dyn model_707::ModelAdapter))
    }

    fn model_708_adapter(&self) -> Option<&'a dyn model_708::ModelAdapter> {
        self.model_708_callback_adapter
        .map(|a| a as &'a dyn model_708::ModelAdapter)
        .or(self.model_708_stateful_adapter.map(|a| a as &'a dyn model_708::ModelAdapter))
    }

    fn model_709_adapter(&self) -> Option<&'a dyn model_709::ModelAdapter> {
        self.model_709_callback_adapter
        .map(|a| a as &'a dyn model_709::ModelAdapter)
        .or(self.model_709_stateful_adapter.map(|a| a as &'a dyn model_709::ModelAdapter))
    }

    fn model_710_adapter(&self) -> Option<&'a dyn model_710::ModelAdapter> {
        self.model_710_callback_adapter
        .map(|a| a as &'a dyn model_710::ModelAdapter)
        .or(self.model_710_stateful_adapter.map(|a| a as &'a dyn model_710::ModelAdapter))
    }

    fn model_711_adapter(&self) -> Option<&'a dyn model_711::ModelAdapter> {
        self.model_711_callback_adapter
        .map(|a| a as &'a dyn model_711::ModelAdapter)
        .or(self.model_711_stateful_adapter.map(|a| a as &'a dyn model_711::ModelAdapter))
    }

    fn model_712_adapter(&self) -> Option<&'a dyn model_712::ModelAdapter> {
        self.model_712_callback_adapter
        .map(|a| a as &'a dyn model_712::ModelAdapter)
        .or(self.model_712_stateful_adapter.map(|a| a as &'a dyn model_712::ModelAdapter))
    }

    fn model_713_adapter(&self) -> Option<&'a dyn model_713::ModelAdapter> {
        self.model_713_callback_adapter
        .map(|a| a as &'a dyn model_713::ModelAdapter)
        .or(self.model_713_stateful_adapter.map(|a| a as &'a dyn model_713::ModelAdapter))
    }

    fn model_714_adapter(&self) -> Option<&'a dyn model_714::ModelAdapter> {
        self.model_714_callback_adapter
        .map(|a| a as &'a dyn model_714::ModelAdapter)
        .or(self.model_714_stateful_adapter.map(|a| a as &'a dyn model_714::ModelAdapter))
    }

    fn model_715_adapter(&self) -> Option<&'a dyn model_715::ModelAdapter> {
        self.model_715_callback_adapter
        .map(|a| a as &'a dyn model_715::ModelAdapter)
        .or(self.model_715_stateful_adapter.map(|a| a as &'a dyn model_715::ModelAdapter))
    }

    fn model_801_adapter(&self) -> Option<&'a dyn model_801::ModelAdapter> {
        self.model_801_callback_adapter
        .map(|a| a as &'a dyn model_801::ModelAdapter)
        .or(self.model_801_stateful_adapter.map(|a| a as &'a dyn model_801::ModelAdapter))
    }

    fn model_802_adapter(&self) -> Option<&'a dyn model_802::ModelAdapter> {
        self.model_802_callback_adapter
        .map(|a| a as &'a dyn model_802::ModelAdapter)
        .or(self.model_802_stateful_adapter.map(|a| a as &'a dyn model_802::ModelAdapter))
    }

    fn model_803_adapter(&self) -> Option<&'a dyn model_803::ModelAdapter> {
        self.model_803_callback_adapter
        .map(|a| a as &'a dyn model_803::ModelAdapter)
        .or(self.model_803_stateful_adapter.map(|a| a as &'a dyn model_803::ModelAdapter))
    }

    fn model_804_adapter(&self) -> Option<&'a dyn model_804::ModelAdapter> {
        self.model_804_callback_adapter
        .map(|a| a as &'a dyn model_804::ModelAdapter)
        .or(self.model_804_stateful_adapter.map(|a| a as &'a dyn model_804::ModelAdapter))
    }

    fn model_805_adapter(&self) -> Option<&'a dyn model_805::ModelAdapter> {
        self.model_805_callback_adapter
        .map(|a| a as &'a dyn model_805::ModelAdapter)
        .or(self.model_805_stateful_adapter.map(|a| a as &'a dyn model_805::ModelAdapter))
    }

    fn model_806_adapter(&self) -> Option<&'a dyn model_806::ModelAdapter> {
        self.model_806_callback_adapter
        .map(|a| a as &'a dyn model_806::ModelAdapter)
        .or(self.model_806_stateful_adapter.map(|a| a as &'a dyn model_806::ModelAdapter))
    }

    fn model_807_adapter(&self) -> Option<&'a dyn model_807::ModelAdapter> {
        self.model_807_callback_adapter
        .map(|a| a as &'a dyn model_807::ModelAdapter)
        .or(self.model_807_stateful_adapter.map(|a| a as &'a dyn model_807::ModelAdapter))
    }

    fn model_808_adapter(&self) -> Option<&'a dyn model_808::ModelAdapter> {
        self.model_808_callback_adapter
        .map(|a| a as &'a dyn model_808::ModelAdapter)
        .or(self.model_808_stateful_adapter.map(|a| a as &'a dyn model_808::ModelAdapter))
    }

    fn model_809_adapter(&self) -> Option<&'a dyn model_809::ModelAdapter> {
        self.model_809_callback_adapter
        .map(|a| a as &'a dyn model_809::ModelAdapter)
        .or(self.model_809_stateful_adapter.map(|a| a as &'a dyn model_809::ModelAdapter))
    }

    fn model_63001_adapter(&self) -> Option<&'a dyn model_63001::ModelAdapter> {
        self.model_63001_callback_adapter
        .map(|a| a as &'a dyn model_63001::ModelAdapter)
        .or(self.model_63001_stateful_adapter.map(|a| a as &'a dyn model_63001::ModelAdapter))
    }

    fn model_64001_adapter(&self) -> Option<&'a dyn model_64001::ModelAdapter> {
        self.model_64001_callback_adapter
        .map(|a| a as &'a dyn model_64001::ModelAdapter)
        .or(self.model_64001_stateful_adapter.map(|a| a as &'a dyn model_64001::ModelAdapter))
    }

    fn model_64020_adapter(&self) -> Option<&'a dyn model_64020::ModelAdapter> {
        self.model_64020_callback_adapter
        .map(|a| a as &'a dyn model_64020::ModelAdapter)
        .or(self.model_64020_stateful_adapter.map(|a| a as &'a dyn model_64020::ModelAdapter))
    }

    fn model_64101_adapter(&self) -> Option<&'a dyn model_64101::ModelAdapter> {
        self.model_64101_callback_adapter
        .map(|a| a as &'a dyn model_64101::ModelAdapter)
        .or(self.model_64101_stateful_adapter.map(|a| a as &'a dyn model_64101::ModelAdapter))
    }

    fn model_64111_adapter(&self) -> Option<&'a dyn model_64111::ModelAdapter> {
        self.model_64111_callback_adapter
        .map(|a| a as &'a dyn model_64111::ModelAdapter)
        .or(self.model_64111_stateful_adapter.map(|a| a as &'a dyn model_64111::ModelAdapter))
    }

    fn model_64112_adapter(&self) -> Option<&'a dyn model_64112::ModelAdapter> {
        self.model_64112_callback_adapter
        .map(|a| a as &'a dyn model_64112::ModelAdapter)
        .or(self.model_64112_stateful_adapter.map(|a| a as &'a dyn model_64112::ModelAdapter))
    }

    fn model_64410_adapter(&self) -> Option<&'a dyn model_64410::ModelAdapter> {
        self.model_64410_callback_adapter
        .map(|a| a as &'a dyn model_64410::ModelAdapter)
        .or(self.model_64410_stateful_adapter.map(|a| a as &'a dyn model_64410::ModelAdapter))
    }

    fn model_64411_adapter(&self) -> Option<&'a dyn model_64411::ModelAdapter> {
        self.model_64411_callback_adapter
        .map(|a| a as &'a dyn model_64411::ModelAdapter)
        .or(self.model_64411_stateful_adapter.map(|a| a as &'a dyn model_64411::ModelAdapter))
    }

    fn model_64412_adapter(&self) -> Option<&'a dyn model_64412::ModelAdapter> {
        self.model_64412_callback_adapter
        .map(|a| a as &'a dyn model_64412::ModelAdapter)
        .or(self.model_64412_stateful_adapter.map(|a| a as &'a dyn model_64412::ModelAdapter))
    }

    fn model_64413_adapter(&self) -> Option<&'a dyn model_64413::ModelAdapter> {
        self.model_64413_callback_adapter
        .map(|a| a as &'a dyn model_64413::ModelAdapter)
        .or(self.model_64413_stateful_adapter.map(|a| a as &'a dyn model_64413::ModelAdapter))
    }

    fn model_64414_adapter(&self) -> Option<&'a dyn model_64414::ModelAdapter> {
        self.model_64414_callback_adapter
        .map(|a| a as &'a dyn model_64414::ModelAdapter)
        .or(self.model_64414_stateful_adapter.map(|a| a as &'a dyn model_64414::ModelAdapter))
    }

    fn model_64415_adapter(&self) -> Option<&'a dyn model_64415::ModelAdapter> {
        self.model_64415_callback_adapter
        .map(|a| a as &'a dyn model_64415::ModelAdapter)
        .or(self.model_64415_stateful_adapter.map(|a| a as &'a dyn model_64415::ModelAdapter))
    }
}

pub fn points_array_and_offset<'a>(adapters: &'a dyn SunspecAdapterProvider<'a>, address: u16) -> Option<(&'a [ReadablePoint], u16)> {
    let mut offset = address;
    if offset < 2 {
        return Some((&crate::HEADER_POINTS as &'a [ReadablePoint], offset));
    }
    else {
        offset -= 2;
    };
    if adapters.model_1_adapter().is_some() {
        if offset < model_1::SIZE {
            return Some((&model_1::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_1::SIZE;
        }
    }
    if adapters.model_2_adapter().is_some() {
        if offset < model_2::SIZE {
            return Some((&model_2::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_2::SIZE;
        }
    }
    if adapters.model_3_adapter().is_some() {
        if offset < model_3::SIZE {
            return Some((&model_3::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_3::SIZE;
        }
    }
    if adapters.model_4_adapter().is_some() {
        if offset < model_4::SIZE {
            return Some((&model_4::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_4::SIZE;
        }
    }
    if adapters.model_5_adapter().is_some() {
        if offset < model_5::SIZE {
            return Some((&model_5::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_5::SIZE;
        }
    }
    if adapters.model_6_adapter().is_some() {
        if offset < model_6::SIZE {
            return Some((&model_6::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_6::SIZE;
        }
    }
    if adapters.model_7_adapter().is_some() {
        if offset < model_7::SIZE {
            return Some((&model_7::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_7::SIZE;
        }
    }
    if adapters.model_8_adapter().is_some() {
        if offset < model_8::SIZE {
            return Some((&model_8::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_8::SIZE;
        }
    }
    if adapters.model_10_adapter().is_some() {
        if offset < model_10::SIZE {
            return Some((&model_10::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_10::SIZE;
        }
    }
    if adapters.model_11_adapter().is_some() {
        if offset < model_11::SIZE {
            return Some((&model_11::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_11::SIZE;
        }
    }
    if adapters.model_12_adapter().is_some() {
        if offset < model_12::SIZE {
            return Some((&model_12::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_12::SIZE;
        }
    }
    if adapters.model_13_adapter().is_some() {
        if offset < model_13::SIZE {
            return Some((&model_13::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_13::SIZE;
        }
    }
    if adapters.model_15_adapter().is_some() {
        if offset < model_15::SIZE {
            return Some((&model_15::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_15::SIZE;
        }
    }
    if adapters.model_16_adapter().is_some() {
        if offset < model_16::SIZE {
            return Some((&model_16::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_16::SIZE;
        }
    }
    if adapters.model_17_adapter().is_some() {
        if offset < model_17::SIZE {
            return Some((&model_17::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_17::SIZE;
        }
    }
    if adapters.model_18_adapter().is_some() {
        if offset < model_18::SIZE {
            return Some((&model_18::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_18::SIZE;
        }
    }
    if adapters.model_19_adapter().is_some() {
        if offset < model_19::SIZE {
            return Some((&model_19::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_19::SIZE;
        }
    }
    if adapters.model_101_adapter().is_some() {
        if offset < model_101::SIZE {
            return Some((&model_101::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_101::SIZE;
        }
    }
    if adapters.model_102_adapter().is_some() {
        if offset < model_102::SIZE {
            return Some((&model_102::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_102::SIZE;
        }
    }
    if adapters.model_103_adapter().is_some() {
        if offset < model_103::SIZE {
            return Some((&model_103::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_103::SIZE;
        }
    }
    if adapters.model_111_adapter().is_some() {
        if offset < model_111::SIZE {
            return Some((&model_111::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_111::SIZE;
        }
    }
    if adapters.model_112_adapter().is_some() {
        if offset < model_112::SIZE {
            return Some((&model_112::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_112::SIZE;
        }
    }
    if adapters.model_113_adapter().is_some() {
        if offset < model_113::SIZE {
            return Some((&model_113::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_113::SIZE;
        }
    }
    if adapters.model_120_adapter().is_some() {
        if offset < model_120::SIZE {
            return Some((&model_120::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_120::SIZE;
        }
    }
    if adapters.model_121_adapter().is_some() {
        if offset < model_121::SIZE {
            return Some((&model_121::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_121::SIZE;
        }
    }
    if adapters.model_122_adapter().is_some() {
        if offset < model_122::SIZE {
            return Some((&model_122::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_122::SIZE;
        }
    }
    if adapters.model_123_adapter().is_some() {
        if offset < model_123::SIZE {
            return Some((&model_123::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_123::SIZE;
        }
    }
    if adapters.model_124_adapter().is_some() {
        if offset < model_124::SIZE {
            return Some((&model_124::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_124::SIZE;
        }
    }
    if adapters.model_125_adapter().is_some() {
        if offset < model_125::SIZE {
            return Some((&model_125::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_125::SIZE;
        }
    }
    if adapters.model_126_adapter().is_some() {
        if offset < model_126::SIZE {
            return Some((&model_126::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_126::SIZE;
        }
    }
    if adapters.model_127_adapter().is_some() {
        if offset < model_127::SIZE {
            return Some((&model_127::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_127::SIZE;
        }
    }
    if adapters.model_128_adapter().is_some() {
        if offset < model_128::SIZE {
            return Some((&model_128::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_128::SIZE;
        }
    }
    if adapters.model_129_adapter().is_some() {
        if offset < model_129::SIZE {
            return Some((&model_129::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_129::SIZE;
        }
    }
    if adapters.model_130_adapter().is_some() {
        if offset < model_130::SIZE {
            return Some((&model_130::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_130::SIZE;
        }
    }
    if adapters.model_131_adapter().is_some() {
        if offset < model_131::SIZE {
            return Some((&model_131::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_131::SIZE;
        }
    }
    if adapters.model_132_adapter().is_some() {
        if offset < model_132::SIZE {
            return Some((&model_132::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_132::SIZE;
        }
    }
    if adapters.model_133_adapter().is_some() {
        if offset < model_133::SIZE {
            return Some((&model_133::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_133::SIZE;
        }
    }
    if adapters.model_134_adapter().is_some() {
        if offset < model_134::SIZE {
            return Some((&model_134::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_134::SIZE;
        }
    }
    if adapters.model_135_adapter().is_some() {
        if offset < model_135::SIZE {
            return Some((&model_135::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_135::SIZE;
        }
    }
    if adapters.model_136_adapter().is_some() {
        if offset < model_136::SIZE {
            return Some((&model_136::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_136::SIZE;
        }
    }
    if adapters.model_137_adapter().is_some() {
        if offset < model_137::SIZE {
            return Some((&model_137::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_137::SIZE;
        }
    }
    if adapters.model_138_adapter().is_some() {
        if offset < model_138::SIZE {
            return Some((&model_138::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_138::SIZE;
        }
    }
    if adapters.model_139_adapter().is_some() {
        if offset < model_139::SIZE {
            return Some((&model_139::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_139::SIZE;
        }
    }
    if adapters.model_140_adapter().is_some() {
        if offset < model_140::SIZE {
            return Some((&model_140::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_140::SIZE;
        }
    }
    if adapters.model_141_adapter().is_some() {
        if offset < model_141::SIZE {
            return Some((&model_141::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_141::SIZE;
        }
    }
    if adapters.model_142_adapter().is_some() {
        if offset < model_142::SIZE {
            return Some((&model_142::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_142::SIZE;
        }
    }
    if adapters.model_143_adapter().is_some() {
        if offset < model_143::SIZE {
            return Some((&model_143::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_143::SIZE;
        }
    }
    if adapters.model_144_adapter().is_some() {
        if offset < model_144::SIZE {
            return Some((&model_144::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_144::SIZE;
        }
    }
    if adapters.model_145_adapter().is_some() {
        if offset < model_145::SIZE {
            return Some((&model_145::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_145::SIZE;
        }
    }
    if adapters.model_160_adapter().is_some() {
        if offset < model_160::SIZE {
            return Some((&model_160::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_160::SIZE;
        }
    }
    if adapters.model_201_adapter().is_some() {
        if offset < model_201::SIZE {
            return Some((&model_201::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_201::SIZE;
        }
    }
    if adapters.model_202_adapter().is_some() {
        if offset < model_202::SIZE {
            return Some((&model_202::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_202::SIZE;
        }
    }
    if adapters.model_203_adapter().is_some() {
        if offset < model_203::SIZE {
            return Some((&model_203::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_203::SIZE;
        }
    }
    if adapters.model_204_adapter().is_some() {
        if offset < model_204::SIZE {
            return Some((&model_204::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_204::SIZE;
        }
    }
    if adapters.model_211_adapter().is_some() {
        if offset < model_211::SIZE {
            return Some((&model_211::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_211::SIZE;
        }
    }
    if adapters.model_212_adapter().is_some() {
        if offset < model_212::SIZE {
            return Some((&model_212::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_212::SIZE;
        }
    }
    if adapters.model_213_adapter().is_some() {
        if offset < model_213::SIZE {
            return Some((&model_213::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_213::SIZE;
        }
    }
    if adapters.model_214_adapter().is_some() {
        if offset < model_214::SIZE {
            return Some((&model_214::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_214::SIZE;
        }
    }
    if adapters.model_220_adapter().is_some() {
        if offset < model_220::SIZE {
            return Some((&model_220::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_220::SIZE;
        }
    }
    if adapters.model_305_adapter().is_some() {
        if offset < model_305::SIZE {
            return Some((&model_305::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_305::SIZE;
        }
    }
    if adapters.model_306_adapter().is_some() {
        if offset < model_306::SIZE {
            return Some((&model_306::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_306::SIZE;
        }
    }
    if adapters.model_307_adapter().is_some() {
        if offset < model_307::SIZE {
            return Some((&model_307::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_307::SIZE;
        }
    }
    if adapters.model_308_adapter().is_some() {
        if offset < model_308::SIZE {
            return Some((&model_308::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_308::SIZE;
        }
    }
    if adapters.model_401_adapter().is_some() {
        if offset < model_401::SIZE {
            return Some((&model_401::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_401::SIZE;
        }
    }
    if adapters.model_402_adapter().is_some() {
        if offset < model_402::SIZE {
            return Some((&model_402::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_402::SIZE;
        }
    }
    if adapters.model_403_adapter().is_some() {
        if offset < model_403::SIZE {
            return Some((&model_403::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_403::SIZE;
        }
    }
    if adapters.model_404_adapter().is_some() {
        if offset < model_404::SIZE {
            return Some((&model_404::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_404::SIZE;
        }
    }
    if adapters.model_501_adapter().is_some() {
        if offset < model_501::SIZE {
            return Some((&model_501::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_501::SIZE;
        }
    }
    if adapters.model_502_adapter().is_some() {
        if offset < model_502::SIZE {
            return Some((&model_502::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_502::SIZE;
        }
    }
    if adapters.model_701_adapter().is_some() {
        if offset < model_701::SIZE {
            return Some((&model_701::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_701::SIZE;
        }
    }
    if adapters.model_703_adapter().is_some() {
        if offset < model_703::SIZE {
            return Some((&model_703::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_703::SIZE;
        }
    }
    if adapters.model_704_adapter().is_some() {
        if offset < model_704::SIZE {
            return Some((&model_704::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_704::SIZE;
        }
    }
    if adapters.model_705_adapter().is_some() {
        if offset < model_705::SIZE {
            return Some((&model_705::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_705::SIZE;
        }
    }
    if adapters.model_706_adapter().is_some() {
        if offset < model_706::SIZE {
            return Some((&model_706::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_706::SIZE;
        }
    }
    if adapters.model_707_adapter().is_some() {
        if offset < model_707::SIZE {
            return Some((&model_707::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_707::SIZE;
        }
    }
    if adapters.model_708_adapter().is_some() {
        if offset < model_708::SIZE {
            return Some((&model_708::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_708::SIZE;
        }
    }
    if adapters.model_709_adapter().is_some() {
        if offset < model_709::SIZE {
            return Some((&model_709::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_709::SIZE;
        }
    }
    if adapters.model_710_adapter().is_some() {
        if offset < model_710::SIZE {
            return Some((&model_710::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_710::SIZE;
        }
    }
    if adapters.model_711_adapter().is_some() {
        if offset < model_711::SIZE {
            return Some((&model_711::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_711::SIZE;
        }
    }
    if adapters.model_712_adapter().is_some() {
        if offset < model_712::SIZE {
            return Some((&model_712::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_712::SIZE;
        }
    }
    if adapters.model_713_adapter().is_some() {
        if offset < model_713::SIZE {
            return Some((&model_713::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_713::SIZE;
        }
    }
    if adapters.model_714_adapter().is_some() {
        if offset < model_714::SIZE {
            return Some((&model_714::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_714::SIZE;
        }
    }
    if adapters.model_715_adapter().is_some() {
        if offset < model_715::SIZE {
            return Some((&model_715::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_715::SIZE;
        }
    }
    if adapters.model_801_adapter().is_some() {
        if offset < model_801::SIZE {
            return Some((&model_801::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_801::SIZE;
        }
    }
    if adapters.model_802_adapter().is_some() {
        if offset < model_802::SIZE {
            return Some((&model_802::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_802::SIZE;
        }
    }
    if adapters.model_803_adapter().is_some() {
        if offset < model_803::SIZE {
            return Some((&model_803::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_803::SIZE;
        }
    }
    if adapters.model_804_adapter().is_some() {
        if offset < model_804::SIZE {
            return Some((&model_804::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_804::SIZE;
        }
    }
    if adapters.model_805_adapter().is_some() {
        if offset < model_805::SIZE {
            return Some((&model_805::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_805::SIZE;
        }
    }
    if adapters.model_806_adapter().is_some() {
        if offset < model_806::SIZE {
            return Some((&model_806::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_806::SIZE;
        }
    }
    if adapters.model_807_adapter().is_some() {
        if offset < model_807::SIZE {
            return Some((&model_807::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_807::SIZE;
        }
    }
    if adapters.model_808_adapter().is_some() {
        if offset < model_808::SIZE {
            return Some((&model_808::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_808::SIZE;
        }
    }
    if adapters.model_809_adapter().is_some() {
        if offset < model_809::SIZE {
            return Some((&model_809::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_809::SIZE;
        }
    }
    if adapters.model_63001_adapter().is_some() {
        if offset < model_63001::SIZE {
            return Some((&model_63001::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_63001::SIZE;
        }
    }
    if adapters.model_64001_adapter().is_some() {
        if offset < model_64001::SIZE {
            return Some((&model_64001::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64001::SIZE;
        }
    }
    if adapters.model_64020_adapter().is_some() {
        if offset < model_64020::SIZE {
            return Some((&model_64020::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64020::SIZE;
        }
    }
    if adapters.model_64101_adapter().is_some() {
        if offset < model_64101::SIZE {
            return Some((&model_64101::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64101::SIZE;
        }
    }
    if adapters.model_64111_adapter().is_some() {
        if offset < model_64111::SIZE {
            return Some((&model_64111::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64111::SIZE;
        }
    }
    if adapters.model_64112_adapter().is_some() {
        if offset < model_64112::SIZE {
            return Some((&model_64112::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64112::SIZE;
        }
    }
    if adapters.model_64410_adapter().is_some() {
        if offset < model_64410::SIZE {
            return Some((&model_64410::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64410::SIZE;
        }
    }
    if adapters.model_64411_adapter().is_some() {
        if offset < model_64411::SIZE {
            return Some((&model_64411::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64411::SIZE;
        }
    }
    if adapters.model_64412_adapter().is_some() {
        if offset < model_64412::SIZE {
            return Some((&model_64412::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64412::SIZE;
        }
    }
    if adapters.model_64413_adapter().is_some() {
        if offset < model_64413::SIZE {
            return Some((&model_64413::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64413::SIZE;
        }
    }
    if adapters.model_64414_adapter().is_some() {
        if offset < model_64414::SIZE {
            return Some((&model_64414::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64414::SIZE;
        }
    }
    if adapters.model_64415_adapter().is_some() {
        if offset < model_64415::SIZE {
            return Some((&model_64415::POINTS as &'a [ReadablePoint], offset));
        }
        else {
            offset -= model_64415::SIZE;
        }
    }
    None
}

pub fn write_point<'a>(adapters: &'a dyn SunspecAdapterProvider<'a>, point_ref: &PointReference, buffer: &mut [u16], offset: u16, limit: u16) {
    match point_ref {
        PointReference::Static { value } => { buffer[0] = *value; }
        PointReference::Model1 { point } => {
            model_1::write_point(
            adapters.model_1_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model2 { point } => {
            model_2::write_point(
            adapters.model_2_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model3 { point } => {
            model_3::write_point(
            adapters.model_3_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model4 { point } => {
            model_4::write_point(
            adapters.model_4_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model5 { point } => {
            model_5::write_point(
            adapters.model_5_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model6 { point } => {
            model_6::write_point(
            adapters.model_6_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model7 { point } => {
            model_7::write_point(
            adapters.model_7_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model8 { point } => {
            model_8::write_point(
            adapters.model_8_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model10 { point } => {
            model_10::write_point(
            adapters.model_10_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model11 { point } => {
            model_11::write_point(
            adapters.model_11_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model12 { point } => {
            model_12::write_point(
            adapters.model_12_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model13 { point } => {
            model_13::write_point(
            adapters.model_13_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model15 { point } => {
            model_15::write_point(
            adapters.model_15_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model16 { point } => {
            model_16::write_point(
            adapters.model_16_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model17 { point } => {
            model_17::write_point(
            adapters.model_17_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model18 { point } => {
            model_18::write_point(
            adapters.model_18_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model19 { point } => {
            model_19::write_point(
            adapters.model_19_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model101 { point } => {
            model_101::write_point(
            adapters.model_101_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model102 { point } => {
            model_102::write_point(
            adapters.model_102_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model103 { point } => {
            model_103::write_point(
            adapters.model_103_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model111 { point } => {
            model_111::write_point(
            adapters.model_111_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model112 { point } => {
            model_112::write_point(
            adapters.model_112_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model113 { point } => {
            model_113::write_point(
            adapters.model_113_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model120 { point } => {
            model_120::write_point(
            adapters.model_120_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model121 { point } => {
            model_121::write_point(
            adapters.model_121_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model122 { point } => {
            model_122::write_point(
            adapters.model_122_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model123 { point } => {
            model_123::write_point(
            adapters.model_123_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model124 { point } => {
            model_124::write_point(
            adapters.model_124_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model125 { point } => {
            model_125::write_point(
            adapters.model_125_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model126 { point } => {
            model_126::write_point(
            adapters.model_126_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model127 { point } => {
            model_127::write_point(
            adapters.model_127_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model128 { point } => {
            model_128::write_point(
            adapters.model_128_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model129 { point } => {
            model_129::write_point(
            adapters.model_129_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model130 { point } => {
            model_130::write_point(
            adapters.model_130_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model131 { point } => {
            model_131::write_point(
            adapters.model_131_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model132 { point } => {
            model_132::write_point(
            adapters.model_132_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model133 { point } => {
            model_133::write_point(
            adapters.model_133_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model134 { point } => {
            model_134::write_point(
            adapters.model_134_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model135 { point } => {
            model_135::write_point(
            adapters.model_135_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model136 { point } => {
            model_136::write_point(
            adapters.model_136_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model137 { point } => {
            model_137::write_point(
            adapters.model_137_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model138 { point } => {
            model_138::write_point(
            adapters.model_138_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model139 { point } => {
            model_139::write_point(
            adapters.model_139_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model140 { point } => {
            model_140::write_point(
            adapters.model_140_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model141 { point } => {
            model_141::write_point(
            adapters.model_141_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model142 { point } => {
            model_142::write_point(
            adapters.model_142_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model143 { point } => {
            model_143::write_point(
            adapters.model_143_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model144 { point } => {
            model_144::write_point(
            adapters.model_144_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model145 { point } => {
            model_145::write_point(
            adapters.model_145_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model160 { point } => {
            model_160::write_point(
            adapters.model_160_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model201 { point } => {
            model_201::write_point(
            adapters.model_201_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model202 { point } => {
            model_202::write_point(
            adapters.model_202_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model203 { point } => {
            model_203::write_point(
            adapters.model_203_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model204 { point } => {
            model_204::write_point(
            adapters.model_204_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model211 { point } => {
            model_211::write_point(
            adapters.model_211_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model212 { point } => {
            model_212::write_point(
            adapters.model_212_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model213 { point } => {
            model_213::write_point(
            adapters.model_213_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model214 { point } => {
            model_214::write_point(
            adapters.model_214_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model220 { point } => {
            model_220::write_point(
            adapters.model_220_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model305 { point } => {
            model_305::write_point(
            adapters.model_305_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model306 { point } => {
            model_306::write_point(
            adapters.model_306_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model307 { point } => {
            model_307::write_point(
            adapters.model_307_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model308 { point } => {
            model_308::write_point(
            adapters.model_308_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model401 { point } => {
            model_401::write_point(
            adapters.model_401_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model402 { point } => {
            model_402::write_point(
            adapters.model_402_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model403 { point } => {
            model_403::write_point(
            adapters.model_403_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model404 { point } => {
            model_404::write_point(
            adapters.model_404_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model501 { point } => {
            model_501::write_point(
            adapters.model_501_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model502 { point } => {
            model_502::write_point(
            adapters.model_502_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model701 { point } => {
            model_701::write_point(
            adapters.model_701_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model703 { point } => {
            model_703::write_point(
            adapters.model_703_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model704 { point } => {
            model_704::write_point(
            adapters.model_704_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model705 { point } => {
            model_705::write_point(
            adapters.model_705_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model706 { point } => {
            model_706::write_point(
            adapters.model_706_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model707 { point } => {
            model_707::write_point(
            adapters.model_707_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model708 { point } => {
            model_708::write_point(
            adapters.model_708_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model709 { point } => {
            model_709::write_point(
            adapters.model_709_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model710 { point } => {
            model_710::write_point(
            adapters.model_710_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model711 { point } => {
            model_711::write_point(
            adapters.model_711_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model712 { point } => {
            model_712::write_point(
            adapters.model_712_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model713 { point } => {
            model_713::write_point(
            adapters.model_713_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model714 { point } => {
            model_714::write_point(
            adapters.model_714_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model715 { point } => {
            model_715::write_point(
            adapters.model_715_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model801 { point } => {
            model_801::write_point(
            adapters.model_801_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model802 { point } => {
            model_802::write_point(
            adapters.model_802_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model803 { point } => {
            model_803::write_point(
            adapters.model_803_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model804 { point } => {
            model_804::write_point(
            adapters.model_804_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model805 { point } => {
            model_805::write_point(
            adapters.model_805_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model806 { point } => {
            model_806::write_point(
            adapters.model_806_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model807 { point } => {
            model_807::write_point(
            adapters.model_807_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model808 { point } => {
            model_808::write_point(
            adapters.model_808_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model809 { point } => {
            model_809::write_point(
            adapters.model_809_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model63001 { point } => {
            model_63001::write_point(
            adapters.model_63001_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64001 { point } => {
            model_64001::write_point(
            adapters.model_64001_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64020 { point } => {
            model_64020::write_point(
            adapters.model_64020_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64101 { point } => {
            model_64101::write_point(
            adapters.model_64101_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64111 { point } => {
            model_64111::write_point(
            adapters.model_64111_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64112 { point } => {
            model_64112::write_point(
            adapters.model_64112_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64410 { point } => {
            model_64410::write_point(
            adapters.model_64410_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64411 { point } => {
            model_64411::write_point(
            adapters.model_64411_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64412 { point } => {
            model_64412::write_point(
            adapters.model_64412_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64413 { point } => {
            model_64413::write_point(
            adapters.model_64413_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64414 { point } => {
            model_64414::write_point(
            adapters.model_64414_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
        PointReference::Model64415 { point } => {
            model_64415::write_point(
            adapters.model_64415_adapter().unwrap(),
            point,
            buffer,
            offset,
            limit,
            );
        }
    }
}