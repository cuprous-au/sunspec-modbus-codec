use crate::sunspec::models::{model_1, model_2, model_3, model_4, model_5, model_6, model_7, model_8, model_10, model_11, model_12, model_13, model_15, model_16, model_17, model_18, model_19, model_101, model_102, model_103, model_111, model_112, model_113, model_120, model_121, model_122, model_123, model_124, model_125, model_126, model_127, model_128, model_129, model_130, model_131, model_132, model_133, model_134, model_135, model_136, model_137, model_138, model_139, model_140, model_141, model_142, model_143, model_144, model_145, model_160, model_201, model_202, model_203, model_204, model_211, model_212, model_213, model_214, model_220, model_305, model_306, model_307, model_308, model_401, model_402, model_403, model_404, model_501, model_502, model_701, model_703, model_704, model_705, model_706, model_707, model_708, model_709, model_710, model_711, model_712, model_713, model_714, model_715, model_801, model_802, model_803, model_804, model_805, model_806, model_807, model_808, model_809, model_63001, model_64001, model_64020, model_64101, model_64111, model_64112, model_64410, model_64411, model_64412, model_64413, model_64414, model_64415};

#[derive(Debug)]
pub enum PointReference {
    Model1 {
        point: model_1::Point,
    }
    ,
    Model2 {
        point: model_2::Point,
    }
    ,
    Model3 {
        point: model_3::Point,
    }
    ,
    Model4 {
        point: model_4::Point,
    }
    ,
    Model5 {
        point: model_5::Point,
    }
    ,
    Model6 {
        point: model_6::Point,
    }
    ,
    Model7 {
        point: model_7::Point,
    }
    ,
    Model8 {
        point: model_8::Point,
    }
    ,
    Model10 {
        point: model_10::Point,
    }
    ,
    Model11 {
        point: model_11::Point,
    }
    ,
    Model12 {
        point: model_12::Point,
    }
    ,
    Model13 {
        point: model_13::Point,
    }
    ,
    Model15 {
        point: model_15::Point,
    }
    ,
    Model16 {
        point: model_16::Point,
    }
    ,
    Model17 {
        point: model_17::Point,
    }
    ,
    Model18 {
        point: model_18::Point,
    }
    ,
    Model19 {
        point: model_19::Point,
    }
    ,
    Model101 {
        point: model_101::Point,
    }
    ,
    Model102 {
        point: model_102::Point,
    }
    ,
    Model103 {
        point: model_103::Point,
    }
    ,
    Model111 {
        point: model_111::Point,
    }
    ,
    Model112 {
        point: model_112::Point,
    }
    ,
    Model113 {
        point: model_113::Point,
    }
    ,
    Model120 {
        point: model_120::Point,
    }
    ,
    Model121 {
        point: model_121::Point,
    }
    ,
    Model122 {
        point: model_122::Point,
    }
    ,
    Model123 {
        point: model_123::Point,
    }
    ,
    Model124 {
        point: model_124::Point,
    }
    ,
    Model125 {
        point: model_125::Point,
    }
    ,
    Model126 {
        point: model_126::Point,
    }
    ,
    Model127 {
        point: model_127::Point,
    }
    ,
    Model128 {
        point: model_128::Point,
    }
    ,
    Model129 {
        point: model_129::Point,
    }
    ,
    Model130 {
        point: model_130::Point,
    }
    ,
    Model131 {
        point: model_131::Point,
    }
    ,
    Model132 {
        point: model_132::Point,
    }
    ,
    Model133 {
        point: model_133::Point,
    }
    ,
    Model134 {
        point: model_134::Point,
    }
    ,
    Model135 {
        point: model_135::Point,
    }
    ,
    Model136 {
        point: model_136::Point,
    }
    ,
    Model137 {
        point: model_137::Point,
    }
    ,
    Model138 {
        point: model_138::Point,
    }
    ,
    Model139 {
        point: model_139::Point,
    }
    ,
    Model140 {
        point: model_140::Point,
    }
    ,
    Model141 {
        point: model_141::Point,
    }
    ,
    Model142 {
        point: model_142::Point,
    }
    ,
    Model143 {
        point: model_143::Point,
    }
    ,
    Model144 {
        point: model_144::Point,
    }
    ,
    Model145 {
        point: model_145::Point,
    }
    ,
    Model160 {
        point: model_160::Point,
    }
    ,
    Model201 {
        point: model_201::Point,
    }
    ,
    Model202 {
        point: model_202::Point,
    }
    ,
    Model203 {
        point: model_203::Point,
    }
    ,
    Model204 {
        point: model_204::Point,
    }
    ,
    Model211 {
        point: model_211::Point,
    }
    ,
    Model212 {
        point: model_212::Point,
    }
    ,
    Model213 {
        point: model_213::Point,
    }
    ,
    Model214 {
        point: model_214::Point,
    }
    ,
    Model220 {
        point: model_220::Point,
    }
    ,
    Model305 {
        point: model_305::Point,
    }
    ,
    Model306 {
        point: model_306::Point,
    }
    ,
    Model307 {
        point: model_307::Point,
    }
    ,
    Model308 {
        point: model_308::Point,
    }
    ,
    Model401 {
        point: model_401::Point,
    }
    ,
    Model402 {
        point: model_402::Point,
    }
    ,
    Model403 {
        point: model_403::Point,
    }
    ,
    Model404 {
        point: model_404::Point,
    }
    ,
    Model501 {
        point: model_501::Point,
    }
    ,
    Model502 {
        point: model_502::Point,
    }
    ,
    Model701 {
        point: model_701::Point,
    }
    ,
    Model703 {
        point: model_703::Point,
    }
    ,
    Model704 {
        point: model_704::Point,
    }
    ,
    Model705 {
        point: model_705::Point,
    }
    ,
    Model706 {
        point: model_706::Point,
    }
    ,
    Model707 {
        point: model_707::Point,
    }
    ,
    Model708 {
        point: model_708::Point,
    }
    ,
    Model709 {
        point: model_709::Point,
    }
    ,
    Model710 {
        point: model_710::Point,
    }
    ,
    Model711 {
        point: model_711::Point,
    }
    ,
    Model712 {
        point: model_712::Point,
    }
    ,
    Model713 {
        point: model_713::Point,
    }
    ,
    Model714 {
        point: model_714::Point,
    }
    ,
    Model715 {
        point: model_715::Point,
    }
    ,
    Model801 {
        point: model_801::Point,
    }
    ,
    Model802 {
        point: model_802::Point,
    }
    ,
    Model803 {
        point: model_803::Point,
    }
    ,
    Model804 {
        point: model_804::Point,
    }
    ,
    Model805 {
        point: model_805::Point,
    }
    ,
    Model806 {
        point: model_806::Point,
    }
    ,
    Model807 {
        point: model_807::Point,
    }
    ,
    Model808 {
        point: model_808::Point,
    }
    ,
    Model809 {
        point: model_809::Point,
    }
    ,
    Model63001 {
        point: model_63001::Point,
    }
    ,
    Model64001 {
        point: model_64001::Point,
    }
    ,
    Model64020 {
        point: model_64020::Point,
    }
    ,
    Model64101 {
        point: model_64101::Point,
    }
    ,
    Model64111 {
        point: model_64111::Point,
    }
    ,
    Model64112 {
        point: model_64112::Point,
    }
    ,
    Model64410 {
        point: model_64410::Point,
    }
    ,
    Model64411 {
        point: model_64411::Point,
    }
    ,
    Model64412 {
        point: model_64412::Point,
    }
    ,
    Model64413 {
        point: model_64413::Point,
    }
    ,
    Model64414 {
        point: model_64414::Point,
    }
    ,
    Model64415 {
        point: model_64415::Point,
    }
    ,
    Static {
        value: u16,
    }
    ,
}