use dicom_rs::modules::io::read_dicom;
fn main() {
    let path = "test_data/Anonymized_20250717.dcm";
    if read_dicom(path) {
        println!("Successfully read DICOM file.");
    } else {
        println!("Failed to read DICOM file.");
    }
}
