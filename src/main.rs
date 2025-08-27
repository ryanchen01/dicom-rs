use dicom_rs::modules::io::read_dicom;
fn main() {
    let path = "test_data/Anonymized_20250717.dcm";
    let ds = read_dicom(path);
    println!(
        "Read DICOM: elements={}, pixel_data={} bytes",
        ds.elements().len(),
        ds.pixel_data().map(|p| p.len()).unwrap_or(0)
    );
    // println!("TransferSyntaxUID: {}", ds.get("TransferSyntaxUID").unwrap());
    println!("PatientName: {}", ds.get("PatientName").unwrap());
    println!("Modality: {}", ds.get("Modality").unwrap());
}
