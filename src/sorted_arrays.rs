// MIT License
//
//Copyright (c) 2018 Brett Russell
//
//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:
//
//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.
//
//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

use super::{CountryCode, NUM_COUNTRY_CODES};

// These are sorted in ascending lexicographic order by the full name of the corresponding
// country as given by ISO 3166-1.
pub const CODES_SORTED_DEFAULT: [CountryCode; NUM_COUNTRY_CODES] = [
    CountryCode::AFG,
    CountryCode::ALA,
    CountryCode::ALB,
    CountryCode::DZA,
    CountryCode::ASM,
    CountryCode::AND,
    CountryCode::AGO,
    CountryCode::AIA,
    CountryCode::ATA,
    CountryCode::ATG,
    CountryCode::ARG,
    CountryCode::ARM,
    CountryCode::ABW,
    CountryCode::AUS,
    CountryCode::AUT,
    CountryCode::AZE,
    CountryCode::BHS,
    CountryCode::BHR,
    CountryCode::BGD,
    CountryCode::BRB,
    CountryCode::BLR,
    CountryCode::BEL,
    CountryCode::BLZ,
    CountryCode::BEN,
    CountryCode::BMU,
    CountryCode::BTN,
    CountryCode::BOL,
    CountryCode::BES,
    CountryCode::BIH,
    CountryCode::BWA,
    CountryCode::BVT,
    CountryCode::BRA,
    CountryCode::IOT,
    CountryCode::BRN,
    CountryCode::BGR,
    CountryCode::BFA,
    CountryCode::BDI,
    CountryCode::CPV,
    CountryCode::KHM,
    CountryCode::CMR,
    CountryCode::CAN,
    CountryCode::CYM,
    CountryCode::CAF,
    CountryCode::TCD,
    CountryCode::CHL,
    CountryCode::CHN,
    CountryCode::CXR,
    CountryCode::CCK,
    CountryCode::COL,
    CountryCode::COM,
    CountryCode::COG,
    CountryCode::COD,
    CountryCode::COK,
    CountryCode::CRI,
    CountryCode::CIV,
    CountryCode::HRV,
    CountryCode::CUB,
    CountryCode::CUW,
    CountryCode::CYP,
    CountryCode::CZE,
    CountryCode::DNK,
    CountryCode::DJI,
    CountryCode::DMA,
    CountryCode::DOM,
    CountryCode::ECU,
    CountryCode::EGY,
    CountryCode::SLV,
    CountryCode::GNQ,
    CountryCode::ERI,
    CountryCode::EST,
    CountryCode::ETH,
    CountryCode::FLK,
    CountryCode::FRO,
    CountryCode::FJI,
    CountryCode::FIN,
    CountryCode::FRA,
    CountryCode::GUF,
    CountryCode::PYF,
    CountryCode::ATF,
    CountryCode::GAB,
    CountryCode::GMB,
    CountryCode::GEO,
    CountryCode::DEU,
    CountryCode::GHA,
    CountryCode::GIB,
    CountryCode::GRC,
    CountryCode::GRL,
    CountryCode::GRD,
    CountryCode::GLP,
    CountryCode::GUM,
    CountryCode::GTM,
    CountryCode::GGY,
    CountryCode::GIN,
    CountryCode::GNB,
    CountryCode::GUY,
    CountryCode::HTI,
    CountryCode::HMD,
    CountryCode::VAT,
    CountryCode::HND,
    CountryCode::HKG,
    CountryCode::HUN,
    CountryCode::ISL,
    CountryCode::IND,
    CountryCode::IDN,
    CountryCode::IRN,
    CountryCode::IRQ,
    CountryCode::IRL,
    CountryCode::IMN,
    CountryCode::ISR,
    CountryCode::ITA,
    CountryCode::JAM,
    CountryCode::JPN,
    CountryCode::JEY,
    CountryCode::JOR,
    CountryCode::KAZ,
    CountryCode::KEN,
    CountryCode::KIR,
    CountryCode::PRK,
    CountryCode::KOR,
    CountryCode::KWT,
    CountryCode::KGZ,
    CountryCode::LAO,
    CountryCode::LVA,
    CountryCode::LBN,
    CountryCode::LSO,
    CountryCode::LBR,
    CountryCode::LBY,
    CountryCode::LIE,
    CountryCode::LTU,
    CountryCode::LUX,
    CountryCode::MAC,
    CountryCode::MKD,
    CountryCode::MDG,
    CountryCode::MWI,
    CountryCode::MYS,
    CountryCode::MDV,
    CountryCode::MLI,
    CountryCode::MLT,
    CountryCode::MHL,
    CountryCode::MTQ,
    CountryCode::MRT,
    CountryCode::MUS,
    CountryCode::MYT,
    CountryCode::MEX,
    CountryCode::FSM,
    CountryCode::MDA,
    CountryCode::MCO,
    CountryCode::MNG,
    CountryCode::MNE,
    CountryCode::MSR,
    CountryCode::MAR,
    CountryCode::MOZ,
    CountryCode::MMR,
    CountryCode::NAM,
    CountryCode::NRU,
    CountryCode::NPL,
    CountryCode::NLD,
    CountryCode::NCL,
    CountryCode::NZL,
    CountryCode::NIC,
    CountryCode::NER,
    CountryCode::NGA,
    CountryCode::NIU,
    CountryCode::NFK,
    CountryCode::MNP,
    CountryCode::NOR,
    CountryCode::OMN,
    CountryCode::PAK,
    CountryCode::PLW,
    CountryCode::PSE,
    CountryCode::PAN,
    CountryCode::PNG,
    CountryCode::PRY,
    CountryCode::PER,
    CountryCode::PHL,
    CountryCode::PCN,
    CountryCode::POL,
    CountryCode::PRT,
    CountryCode::PRI,
    CountryCode::QAT,
    CountryCode::REU,
    CountryCode::ROU,
    CountryCode::RUS,
    CountryCode::RWA,
    CountryCode::BLM,
    CountryCode::SHN,
    CountryCode::KNA,
    CountryCode::LCA,
    CountryCode::MAF,
    CountryCode::SPM,
    CountryCode::VCT,
    CountryCode::WSM,
    CountryCode::SMR,
    CountryCode::STP,
    CountryCode::SAU,
    CountryCode::SEN,
    CountryCode::SRB,
    CountryCode::SYC,
    CountryCode::SLE,
    CountryCode::SGP,
    CountryCode::SXM,
    CountryCode::SVK,
    CountryCode::SVN,
    CountryCode::SLB,
    CountryCode::SOM,
    CountryCode::ZAF,
    CountryCode::SGS,
    CountryCode::SSD,
    CountryCode::ESP,
    CountryCode::LKA,
    CountryCode::SDN,
    CountryCode::SUR,
    CountryCode::SJM,
    CountryCode::SWZ,
    CountryCode::SWE,
    CountryCode::CHE,
    CountryCode::SYR,
    CountryCode::TWN,
    CountryCode::TJK,
    CountryCode::TZA,
    CountryCode::THA,
    CountryCode::TLS,
    CountryCode::TGO,
    CountryCode::TKL,
    CountryCode::TON,
    CountryCode::TTO,
    CountryCode::TUN,
    CountryCode::TUR,
    CountryCode::TKM,
    CountryCode::TCA,
    CountryCode::TUV,
    CountryCode::UGA,
    CountryCode::UKR,
    CountryCode::ARE,
    CountryCode::GBR,
    CountryCode::USA,
    CountryCode::UMI,
    CountryCode::URY,
    CountryCode::UZB,
    CountryCode::VUT,
    CountryCode::VEN,
    CountryCode::VNM,
    CountryCode::VGB,
    CountryCode::VIR,
    CountryCode::WLF,
    CountryCode::ESH,
    CountryCode::YEM,
    CountryCode::ZMB,
    CountryCode::ZWE,
];

pub const CODES_SORTED_NUMERIC_ID: [CountryCode; NUM_COUNTRY_CODES] = [
    CountryCode::AFG,
    CountryCode::ALB,
    CountryCode::ATA,
    CountryCode::DZA,
    CountryCode::ASM,
    CountryCode::AND,
    CountryCode::AGO,
    CountryCode::ATG,
    CountryCode::AZE,
    CountryCode::ARG,
    CountryCode::AUS,
    CountryCode::AUT,
    CountryCode::BHS,
    CountryCode::BHR,
    CountryCode::BGD,
    CountryCode::ARM,
    CountryCode::BRB,
    CountryCode::BEL,
    CountryCode::BMU,
    CountryCode::BTN,
    CountryCode::BOL,
    CountryCode::BIH,
    CountryCode::BWA,
    CountryCode::BVT,
    CountryCode::BRA,
    CountryCode::BLZ,
    CountryCode::IOT,
    CountryCode::SLB,
    CountryCode::VGB,
    CountryCode::BRN,
    CountryCode::BGR,
    CountryCode::MMR,
    CountryCode::BDI,
    CountryCode::BLR,
    CountryCode::KHM,
    CountryCode::CMR,
    CountryCode::CAN,
    CountryCode::CPV,
    CountryCode::CYM,
    CountryCode::CAF,
    CountryCode::LKA,
    CountryCode::TCD,
    CountryCode::CHL,
    CountryCode::CHN,
    CountryCode::TWN,
    CountryCode::CXR,
    CountryCode::CCK,
    CountryCode::COL,
    CountryCode::COM,
    CountryCode::MYT,
    CountryCode::COG,
    CountryCode::COD,
    CountryCode::COK,
    CountryCode::CRI,
    CountryCode::HRV,
    CountryCode::CUB,
    CountryCode::CYP,
    CountryCode::CZE,
    CountryCode::BEN,
    CountryCode::DNK,
    CountryCode::DMA,
    CountryCode::DOM,
    CountryCode::ECU,
    CountryCode::SLV,
    CountryCode::GNQ,
    CountryCode::ETH,
    CountryCode::ERI,
    CountryCode::EST,
    CountryCode::FRO,
    CountryCode::FLK,
    CountryCode::SGS,
    CountryCode::FJI,
    CountryCode::FIN,
    CountryCode::ALA,
    CountryCode::FRA,
    CountryCode::GUF,
    CountryCode::PYF,
    CountryCode::ATF,
    CountryCode::DJI,
    CountryCode::GAB,
    CountryCode::GEO,
    CountryCode::GMB,
    CountryCode::PSE,
    CountryCode::DEU,
    CountryCode::GHA,
    CountryCode::GIB,
    CountryCode::KIR,
    CountryCode::GRC,
    CountryCode::GRL,
    CountryCode::GRD,
    CountryCode::GLP,
    CountryCode::GUM,
    CountryCode::GTM,
    CountryCode::GIN,
    CountryCode::GUY,
    CountryCode::HTI,
    CountryCode::HMD,
    CountryCode::VAT,
    CountryCode::HND,
    CountryCode::HKG,
    CountryCode::HUN,
    CountryCode::ISL,
    CountryCode::IND,
    CountryCode::IDN,
    CountryCode::IRN,
    CountryCode::IRQ,
    CountryCode::IRL,
    CountryCode::ISR,
    CountryCode::ITA,
    CountryCode::CIV,
    CountryCode::JAM,
    CountryCode::JPN,
    CountryCode::KAZ,
    CountryCode::JOR,
    CountryCode::KEN,
    CountryCode::PRK,
    CountryCode::KOR,
    CountryCode::KWT,
    CountryCode::KGZ,
    CountryCode::LAO,
    CountryCode::LBN,
    CountryCode::LSO,
    CountryCode::LVA,
    CountryCode::LBR,
    CountryCode::LBY,
    CountryCode::LIE,
    CountryCode::LTU,
    CountryCode::LUX,
    CountryCode::MAC,
    CountryCode::MDG,
    CountryCode::MWI,
    CountryCode::MYS,
    CountryCode::MDV,
    CountryCode::MLI,
    CountryCode::MLT,
    CountryCode::MTQ,
    CountryCode::MRT,
    CountryCode::MUS,
    CountryCode::MEX,
    CountryCode::MCO,
    CountryCode::MNG,
    CountryCode::MDA,
    CountryCode::MNE,
    CountryCode::MSR,
    CountryCode::MAR,
    CountryCode::MOZ,
    CountryCode::OMN,
    CountryCode::NAM,
    CountryCode::NRU,
    CountryCode::NPL,
    CountryCode::NLD,
    CountryCode::CUW,
    CountryCode::ABW,
    CountryCode::SXM,
    CountryCode::BES,
    CountryCode::NCL,
    CountryCode::VUT,
    CountryCode::NZL,
    CountryCode::NIC,
    CountryCode::NER,
    CountryCode::NGA,
    CountryCode::NIU,
    CountryCode::NFK,
    CountryCode::NOR,
    CountryCode::MNP,
    CountryCode::UMI,
    CountryCode::FSM,
    CountryCode::MHL,
    CountryCode::PLW,
    CountryCode::PAK,
    CountryCode::PAN,
    CountryCode::PNG,
    CountryCode::PRY,
    CountryCode::PER,
    CountryCode::PHL,
    CountryCode::PCN,
    CountryCode::POL,
    CountryCode::PRT,
    CountryCode::GNB,
    CountryCode::TLS,
    CountryCode::PRI,
    CountryCode::QAT,
    CountryCode::REU,
    CountryCode::ROU,
    CountryCode::RUS,
    CountryCode::RWA,
    CountryCode::BLM,
    CountryCode::SHN,
    CountryCode::KNA,
    CountryCode::AIA,
    CountryCode::LCA,
    CountryCode::MAF,
    CountryCode::SPM,
    CountryCode::VCT,
    CountryCode::SMR,
    CountryCode::STP,
    CountryCode::SAU,
    CountryCode::SEN,
    CountryCode::SRB,
    CountryCode::SYC,
    CountryCode::SLE,
    CountryCode::SGP,
    CountryCode::SVK,
    CountryCode::VNM,
    CountryCode::SVN,
    CountryCode::SOM,
    CountryCode::ZAF,
    CountryCode::ZWE,
    CountryCode::ESP,
    CountryCode::SSD,
    CountryCode::SDN,
    CountryCode::ESH,
    CountryCode::SUR,
    CountryCode::SJM,
    CountryCode::SWZ,
    CountryCode::SWE,
    CountryCode::CHE,
    CountryCode::SYR,
    CountryCode::TJK,
    CountryCode::THA,
    CountryCode::TGO,
    CountryCode::TKL,
    CountryCode::TON,
    CountryCode::TTO,
    CountryCode::ARE,
    CountryCode::TUN,
    CountryCode::TUR,
    CountryCode::TKM,
    CountryCode::TCA,
    CountryCode::TUV,
    CountryCode::UGA,
    CountryCode::UKR,
    CountryCode::MKD,
    CountryCode::EGY,
    CountryCode::GBR,
    CountryCode::GGY,
    CountryCode::JEY,
    CountryCode::IMN,
    CountryCode::TZA,
    CountryCode::USA,
    CountryCode::VIR,
    CountryCode::BFA,
    CountryCode::URY,
    CountryCode::UZB,
    CountryCode::VEN,
    CountryCode::WLF,
    CountryCode::WSM,
    CountryCode::YEM,
    CountryCode::ZMB,
];

pub const CODES_SORTED_ALPHA2: [CountryCode; NUM_COUNTRY_CODES] = [
    CountryCode::AND,
    CountryCode::ARE,
    CountryCode::AFG,
    CountryCode::ATG,
    CountryCode::AIA,
    CountryCode::ALB,
    CountryCode::ARM,
    CountryCode::AGO,
    CountryCode::ATA,
    CountryCode::ARG,
    CountryCode::ASM,
    CountryCode::AUT,
    CountryCode::AUS,
    CountryCode::ABW,
    CountryCode::ALA,
    CountryCode::AZE,
    CountryCode::BIH,
    CountryCode::BRB,
    CountryCode::BGD,
    CountryCode::BEL,
    CountryCode::BFA,
    CountryCode::BGR,
    CountryCode::BHR,
    CountryCode::BDI,
    CountryCode::BEN,
    CountryCode::BLM,
    CountryCode::BMU,
    CountryCode::BRN,
    CountryCode::BOL,
    CountryCode::BES,
    CountryCode::BRA,
    CountryCode::BHS,
    CountryCode::BTN,
    CountryCode::BVT,
    CountryCode::BWA,
    CountryCode::BLR,
    CountryCode::BLZ,
    CountryCode::CAN,
    CountryCode::CCK,
    CountryCode::COD,
    CountryCode::CAF,
    CountryCode::COG,
    CountryCode::CHE,
    CountryCode::CIV,
    CountryCode::COK,
    CountryCode::CHL,
    CountryCode::CMR,
    CountryCode::CHN,
    CountryCode::COL,
    CountryCode::CRI,
    CountryCode::CUB,
    CountryCode::CPV,
    CountryCode::CUW,
    CountryCode::CXR,
    CountryCode::CYP,
    CountryCode::CZE,
    CountryCode::DEU,
    CountryCode::DJI,
    CountryCode::DNK,
    CountryCode::DMA,
    CountryCode::DOM,
    CountryCode::DZA,
    CountryCode::ECU,
    CountryCode::EST,
    CountryCode::EGY,
    CountryCode::ESH,
    CountryCode::ERI,
    CountryCode::ESP,
    CountryCode::ETH,
    CountryCode::FIN,
    CountryCode::FJI,
    CountryCode::FLK,
    CountryCode::FSM,
    CountryCode::FRO,
    CountryCode::FRA,
    CountryCode::GAB,
    CountryCode::GBR,
    CountryCode::GRD,
    CountryCode::GEO,
    CountryCode::GUF,
    CountryCode::GGY,
    CountryCode::GHA,
    CountryCode::GIB,
    CountryCode::GRL,
    CountryCode::GMB,
    CountryCode::GIN,
    CountryCode::GLP,
    CountryCode::GNQ,
    CountryCode::GRC,
    CountryCode::SGS,
    CountryCode::GTM,
    CountryCode::GUM,
    CountryCode::GNB,
    CountryCode::GUY,
    CountryCode::HKG,
    CountryCode::HMD,
    CountryCode::HND,
    CountryCode::HRV,
    CountryCode::HTI,
    CountryCode::HUN,
    CountryCode::IDN,
    CountryCode::IRL,
    CountryCode::ISR,
    CountryCode::IMN,
    CountryCode::IND,
    CountryCode::IOT,
    CountryCode::IRQ,
    CountryCode::IRN,
    CountryCode::ISL,
    CountryCode::ITA,
    CountryCode::JEY,
    CountryCode::JAM,
    CountryCode::JOR,
    CountryCode::JPN,
    CountryCode::KEN,
    CountryCode::KGZ,
    CountryCode::KHM,
    CountryCode::KIR,
    CountryCode::COM,
    CountryCode::KNA,
    CountryCode::PRK,
    CountryCode::KOR,
    CountryCode::KWT,
    CountryCode::CYM,
    CountryCode::KAZ,
    CountryCode::LAO,
    CountryCode::LBN,
    CountryCode::LCA,
    CountryCode::LIE,
    CountryCode::LKA,
    CountryCode::LBR,
    CountryCode::LSO,
    CountryCode::LTU,
    CountryCode::LUX,
    CountryCode::LVA,
    CountryCode::LBY,
    CountryCode::MAR,
    CountryCode::MCO,
    CountryCode::MDA,
    CountryCode::MNE,
    CountryCode::MAF,
    CountryCode::MDG,
    CountryCode::MHL,
    CountryCode::MKD,
    CountryCode::MLI,
    CountryCode::MMR,
    CountryCode::MNG,
    CountryCode::MAC,
    CountryCode::MNP,
    CountryCode::MTQ,
    CountryCode::MRT,
    CountryCode::MSR,
    CountryCode::MLT,
    CountryCode::MUS,
    CountryCode::MDV,
    CountryCode::MWI,
    CountryCode::MEX,
    CountryCode::MYS,
    CountryCode::MOZ,
    CountryCode::NAM,
    CountryCode::NCL,
    CountryCode::NER,
    CountryCode::NFK,
    CountryCode::NGA,
    CountryCode::NIC,
    CountryCode::NLD,
    CountryCode::NOR,
    CountryCode::NPL,
    CountryCode::NRU,
    CountryCode::NIU,
    CountryCode::NZL,
    CountryCode::OMN,
    CountryCode::PAN,
    CountryCode::PER,
    CountryCode::PYF,
    CountryCode::PNG,
    CountryCode::PHL,
    CountryCode::PAK,
    CountryCode::POL,
    CountryCode::SPM,
    CountryCode::PCN,
    CountryCode::PRI,
    CountryCode::PSE,
    CountryCode::PRT,
    CountryCode::PLW,
    CountryCode::PRY,
    CountryCode::QAT,
    CountryCode::REU,
    CountryCode::ROU,
    CountryCode::SRB,
    CountryCode::RUS,
    CountryCode::RWA,
    CountryCode::SAU,
    CountryCode::SLB,
    CountryCode::SYC,
    CountryCode::SDN,
    CountryCode::SWE,
    CountryCode::SGP,
    CountryCode::SHN,
    CountryCode::SVN,
    CountryCode::SJM,
    CountryCode::SVK,
    CountryCode::SLE,
    CountryCode::SMR,
    CountryCode::SEN,
    CountryCode::SOM,
    CountryCode::SUR,
    CountryCode::SSD,
    CountryCode::STP,
    CountryCode::SLV,
    CountryCode::SXM,
    CountryCode::SYR,
    CountryCode::SWZ,
    CountryCode::TCA,
    CountryCode::TCD,
    CountryCode::ATF,
    CountryCode::TGO,
    CountryCode::THA,
    CountryCode::TJK,
    CountryCode::TKL,
    CountryCode::TLS,
    CountryCode::TKM,
    CountryCode::TUN,
    CountryCode::TON,
    CountryCode::TUR,
    CountryCode::TTO,
    CountryCode::TUV,
    CountryCode::TWN,
    CountryCode::TZA,
    CountryCode::UKR,
    CountryCode::UGA,
    CountryCode::UMI,
    CountryCode::USA,
    CountryCode::URY,
    CountryCode::UZB,
    CountryCode::VAT,
    CountryCode::VCT,
    CountryCode::VEN,
    CountryCode::VGB,
    CountryCode::VIR,
    CountryCode::VNM,
    CountryCode::VUT,
    CountryCode::WLF,
    CountryCode::WSM,
    CountryCode::YEM,
    CountryCode::MYT,
    CountryCode::ZAF,
    CountryCode::ZMB,
    CountryCode::ZWE,
];

pub const CODES_SORTED_ALPHA3: [CountryCode; NUM_COUNTRY_CODES] = [
    CountryCode::ABW,
    CountryCode::AFG,
    CountryCode::AGO,
    CountryCode::AIA,
    CountryCode::ALA,
    CountryCode::ALB,
    CountryCode::AND,
    CountryCode::ARE,
    CountryCode::ARG,
    CountryCode::ARM,
    CountryCode::ASM,
    CountryCode::ATA,
    CountryCode::ATF,
    CountryCode::ATG,
    CountryCode::AUS,
    CountryCode::AUT,
    CountryCode::AZE,
    CountryCode::BDI,
    CountryCode::BEL,
    CountryCode::BEN,
    CountryCode::BES,
    CountryCode::BFA,
    CountryCode::BGD,
    CountryCode::BGR,
    CountryCode::BHR,
    CountryCode::BHS,
    CountryCode::BIH,
    CountryCode::BLM,
    CountryCode::BLR,
    CountryCode::BLZ,
    CountryCode::BMU,
    CountryCode::BOL,
    CountryCode::BRA,
    CountryCode::BRB,
    CountryCode::BRN,
    CountryCode::BTN,
    CountryCode::BVT,
    CountryCode::BWA,
    CountryCode::CAF,
    CountryCode::CAN,
    CountryCode::CCK,
    CountryCode::CHE,
    CountryCode::CHL,
    CountryCode::CHN,
    CountryCode::CIV,
    CountryCode::CMR,
    CountryCode::COD,
    CountryCode::COG,
    CountryCode::COK,
    CountryCode::COL,
    CountryCode::COM,
    CountryCode::CPV,
    CountryCode::CRI,
    CountryCode::CUB,
    CountryCode::CUW,
    CountryCode::CXR,
    CountryCode::CYM,
    CountryCode::CYP,
    CountryCode::CZE,
    CountryCode::DEU,
    CountryCode::DJI,
    CountryCode::DMA,
    CountryCode::DNK,
    CountryCode::DOM,
    CountryCode::DZA,
    CountryCode::ECU,
    CountryCode::EGY,
    CountryCode::ERI,
    CountryCode::ESH,
    CountryCode::ESP,
    CountryCode::EST,
    CountryCode::ETH,
    CountryCode::FIN,
    CountryCode::FJI,
    CountryCode::FLK,
    CountryCode::FRA,
    CountryCode::FRO,
    CountryCode::FSM,
    CountryCode::GAB,
    CountryCode::GBR,
    CountryCode::GEO,
    CountryCode::GGY,
    CountryCode::GHA,
    CountryCode::GIB,
    CountryCode::GIN,
    CountryCode::GLP,
    CountryCode::GMB,
    CountryCode::GNB,
    CountryCode::GNQ,
    CountryCode::GRC,
    CountryCode::GRD,
    CountryCode::GRL,
    CountryCode::GTM,
    CountryCode::GUF,
    CountryCode::GUM,
    CountryCode::GUY,
    CountryCode::HKG,
    CountryCode::HMD,
    CountryCode::HND,
    CountryCode::HRV,
    CountryCode::HTI,
    CountryCode::HUN,
    CountryCode::IDN,
    CountryCode::IMN,
    CountryCode::IND,
    CountryCode::IOT,
    CountryCode::IRL,
    CountryCode::IRN,
    CountryCode::IRQ,
    CountryCode::ISL,
    CountryCode::ISR,
    CountryCode::ITA,
    CountryCode::JAM,
    CountryCode::JEY,
    CountryCode::JOR,
    CountryCode::JPN,
    CountryCode::KAZ,
    CountryCode::KEN,
    CountryCode::KGZ,
    CountryCode::KHM,
    CountryCode::KIR,
    CountryCode::KNA,
    CountryCode::KOR,
    CountryCode::KWT,
    CountryCode::LAO,
    CountryCode::LBN,
    CountryCode::LBR,
    CountryCode::LBY,
    CountryCode::LCA,
    CountryCode::LIE,
    CountryCode::LKA,
    CountryCode::LSO,
    CountryCode::LTU,
    CountryCode::LUX,
    CountryCode::LVA,
    CountryCode::MAC,
    CountryCode::MAF,
    CountryCode::MAR,
    CountryCode::MCO,
    CountryCode::MDA,
    CountryCode::MDG,
    CountryCode::MDV,
    CountryCode::MEX,
    CountryCode::MHL,
    CountryCode::MKD,
    CountryCode::MLI,
    CountryCode::MLT,
    CountryCode::MMR,
    CountryCode::MNE,
    CountryCode::MNG,
    CountryCode::MNP,
    CountryCode::MOZ,
    CountryCode::MRT,
    CountryCode::MSR,
    CountryCode::MTQ,
    CountryCode::MUS,
    CountryCode::MWI,
    CountryCode::MYS,
    CountryCode::MYT,
    CountryCode::NAM,
    CountryCode::NCL,
    CountryCode::NER,
    CountryCode::NFK,
    CountryCode::NGA,
    CountryCode::NIC,
    CountryCode::NIU,
    CountryCode::NLD,
    CountryCode::NOR,
    CountryCode::NPL,
    CountryCode::NRU,
    CountryCode::NZL,
    CountryCode::OMN,
    CountryCode::PAK,
    CountryCode::PAN,
    CountryCode::PCN,
    CountryCode::PER,
    CountryCode::PHL,
    CountryCode::PLW,
    CountryCode::PNG,
    CountryCode::POL,
    CountryCode::PRI,
    CountryCode::PRK,
    CountryCode::PRT,
    CountryCode::PRY,
    CountryCode::PSE,
    CountryCode::PYF,
    CountryCode::QAT,
    CountryCode::REU,
    CountryCode::ROU,
    CountryCode::RUS,
    CountryCode::RWA,
    CountryCode::SAU,
    CountryCode::SDN,
    CountryCode::SEN,
    CountryCode::SGP,
    CountryCode::SGS,
    CountryCode::SHN,
    CountryCode::SJM,
    CountryCode::SLB,
    CountryCode::SLE,
    CountryCode::SLV,
    CountryCode::SMR,
    CountryCode::SOM,
    CountryCode::SPM,
    CountryCode::SRB,
    CountryCode::SSD,
    CountryCode::STP,
    CountryCode::SUR,
    CountryCode::SVK,
    CountryCode::SVN,
    CountryCode::SWE,
    CountryCode::SWZ,
    CountryCode::SXM,
    CountryCode::SYC,
    CountryCode::SYR,
    CountryCode::TCA,
    CountryCode::TCD,
    CountryCode::TGO,
    CountryCode::THA,
    CountryCode::TJK,
    CountryCode::TKL,
    CountryCode::TKM,
    CountryCode::TLS,
    CountryCode::TON,
    CountryCode::TTO,
    CountryCode::TUN,
    CountryCode::TUR,
    CountryCode::TUV,
    CountryCode::TWN,
    CountryCode::TZA,
    CountryCode::UGA,
    CountryCode::UKR,
    CountryCode::UMI,
    CountryCode::URY,
    CountryCode::USA,
    CountryCode::UZB,
    CountryCode::VAT,
    CountryCode::VCT,
    CountryCode::VEN,
    CountryCode::VGB,
    CountryCode::VIR,
    CountryCode::VNM,
    CountryCode::VUT,
    CountryCode::WLF,
    CountryCode::WSM,
    CountryCode::YEM,
    CountryCode::ZAF,
    CountryCode::ZMB,
    CountryCode::ZWE,
];
