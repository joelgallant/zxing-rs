extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

const ZXING_SRC: &'static str = "zxing/core/src";

fn main() {
    let mut builder = cc::Build::new();
    let mut bindgen_builder = bindgen::builder()
        .header("src/wrapper.hpp")
        .clang_arg(format!("-I{}", ZXING_SRC))
        .clang_arg("-std=c++14")
        .enable_cxx_namespaces()
        .opaque_type("std.*")
        .whitelist_type("zxing::.*");

    let file_list = [
        "bigint/BigInteger.cc",
        "bigint/BigIntegerAlgorithms.cc",
        "bigint/BigIntegerUtils.cc",
        "bigint/BigUnsigned.cc",
        "bigint/BigUnsignedInABase.cc",
        "zxing/aztec/AztecDetectorResult.cpp",
        "zxing/aztec/AztecReader.cpp",
        "zxing/aztec/decoder/Decoder.cpp",
        "zxing/aztec/detector/Detector.cpp",
        "zxing/BarcodeFormat.cpp",
        "zxing/Binarizer.cpp",
        "zxing/BinaryBitmap.cpp",
        "zxing/ChecksumException.cpp",
        "zxing/common/BitArray.cpp",
        "zxing/common/BitArrayIO.cpp",
        "zxing/common/BitMatrix.cpp",
        "zxing/common/BitSource.cpp",
        "zxing/common/CharacterSetECI.cpp",
        "zxing/common/DecoderResult.cpp",
        "zxing/common/detector/MonochromeRectangleDetector.cpp",
        "zxing/common/detector/WhiteRectangleDetector.cpp",
        "zxing/common/DetectorResult.cpp",
        "zxing/common/GlobalHistogramBinarizer.cpp",
        "zxing/common/GreyscaleLuminanceSource.cpp",
        "zxing/common/GreyscaleRotatedLuminanceSource.cpp",
        "zxing/common/GridSampler.cpp",
        "zxing/common/HybridBinarizer.cpp",
        "zxing/common/IllegalArgumentException.cpp",
        "zxing/common/PerspectiveTransform.cpp",
        "zxing/common/reedsolomon/GenericGF.cpp",
        "zxing/common/reedsolomon/GenericGFPoly.cpp",
        "zxing/common/reedsolomon/ReedSolomonDecoder.cpp",
        "zxing/common/reedsolomon/ReedSolomonException.cpp",
        "zxing/common/Str.cpp",
        "zxing/common/StringUtils.cpp",
        "zxing/datamatrix/DataMatrixReader.cpp",
        "zxing/datamatrix/decoder/BitMatrixParser.cpp",
        "zxing/datamatrix/decoder/DataBlock.cpp",
        "zxing/datamatrix/decoder/DecodedBitStreamParser.cpp",
        "zxing/datamatrix/decoder/Decoder.cpp",
        "zxing/datamatrix/detector/CornerPoint.cpp",
        "zxing/datamatrix/detector/Detector.cpp",
        "zxing/datamatrix/detector/DetectorException.cpp",
        "zxing/datamatrix/Version.cpp",
        "zxing/DecodeHints.cpp",
        "zxing/Exception.cpp",
        "zxing/FormatException.cpp",
        "zxing/InvertedLuminanceSource.cpp",
        "zxing/LuminanceSource.cpp",
        "zxing/multi/ByQuadrantReader.cpp",
        "zxing/multi/GenericMultipleBarcodeReader.cpp",
        "zxing/multi/MultipleBarcodeReader.cpp",
        "zxing/multi/qrcode/detector/MultiDetector.cpp",
        "zxing/multi/qrcode/detector/MultiFinderPatternFinder.cpp",
        "zxing/multi/qrcode/QRCodeMultiReader.cpp",
        "zxing/MultiFormatReader.cpp",
        "zxing/oned/CodaBarReader.cpp",
        "zxing/oned/Code128Reader.cpp",
        "zxing/oned/Code39Reader.cpp",
        "zxing/oned/Code93Reader.cpp",
        "zxing/oned/EAN13Reader.cpp",
        "zxing/oned/EAN8Reader.cpp",
        "zxing/oned/ITFReader.cpp",
        "zxing/oned/MultiFormatOneDReader.cpp",
        "zxing/oned/MultiFormatUPCEANReader.cpp",
        "zxing/oned/OneDReader.cpp",
        "zxing/oned/OneDResultPoint.cpp",
        "zxing/oned/UPCAReader.cpp",
        "zxing/oned/UPCEANReader.cpp",
        "zxing/oned/UPCEReader.cpp",
        "zxing/pdf417/decoder/BitMatrixParser.cpp",
        "zxing/pdf417/decoder/DecodedBitStreamParser.cpp",
        "zxing/pdf417/decoder/Decoder.cpp",
        "zxing/pdf417/decoder/ec/ErrorCorrection.cpp",
        "zxing/pdf417/decoder/ec/ModulusGF.cpp",
        "zxing/pdf417/decoder/ec/ModulusPoly.cpp",
        "zxing/pdf417/detector/Detector.cpp",
        "zxing/pdf417/detector/LinesSampler.cpp",
        "zxing/pdf417/PDF417Reader.cpp",
        "zxing/qrcode/decoder/BitMatrixParser.cpp",
        "zxing/qrcode/decoder/DataBlock.cpp",
        "zxing/qrcode/decoder/DataMask.cpp",
        "zxing/qrcode/decoder/DecodedBitStreamParser.cpp",
        "zxing/qrcode/decoder/Decoder.cpp",
        "zxing/qrcode/decoder/Mode.cpp",
        "zxing/qrcode/detector/AlignmentPattern.cpp",
        "zxing/qrcode/detector/AlignmentPatternFinder.cpp",
        "zxing/qrcode/detector/Detector.cpp",
        "zxing/qrcode/detector/FinderPattern.cpp",
        "zxing/qrcode/detector/FinderPatternFinder.cpp",
        "zxing/qrcode/detector/FinderPatternInfo.cpp",
        "zxing/qrcode/ErrorCorrectionLevel.cpp",
        "zxing/qrcode/FormatInformation.cpp",
        "zxing/qrcode/QRCodeReader.cpp",
        "zxing/qrcode/Version.cpp",
        "zxing/Reader.cpp",
        "zxing/Result.cpp",
        "zxing/ResultIO.cpp",
        "zxing/ResultPoint.cpp",
        "zxing/ResultPointCallback.cpp",
    ];

    builder
        .cpp(true)
        .warnings(false)
        .include(ZXING_SRC)
        .files(file_list.iter().map(|f| PathBuf::from(ZXING_SRC).join(f))) ;

    if env::var("TARGET").unwrap() == "armv7-unknown-linux-gnueabihf" {
        let sys_include = [
            "/usr/arm-linux-gnueabihf/include",
            "/usr/arm-linux-gnueabihf/include/c++/8",
            "/usr/arm-linux-gnueabihf/include/c++/8/arm-linux-gnueabihf",
        ];

        for include in sys_include.iter() {
            builder.include(include);
            bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", include));
        }
    }

    builder.compile("zxing");

    bindgen_builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
