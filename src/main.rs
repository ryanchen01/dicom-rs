use dicom_rs::modules::io::read_dicom;
fn main() {
    let path = "test_data/Anonymized_20250717.dcm";
    let ds = read_dicom(path);
    println!(
        "Read DICOM: file_meta={}, elements={}, pixel_data={} bytes",
        ds.file_meta().len(),
        ds.elements().len(),
        ds.pixel_data().map(|p| p.len()).unwrap_or(0)
    );
    println!("-- File Meta --");
    if let Some(e) = ds.get("MediaStorageSOPClassUID") { println!("MediaStorageSOPClassUID: {}", e); }
    if let Some(e) = ds.get("MediaStorageSOPInstanceUID") { println!("MediaStorageSOPInstanceUID: {}", e); }
    if let Some(e) = ds.get("TransferSyntaxUID") { println!("TransferSyntaxUID: {}", e); }
    if let Some(e) = ds.get("ImplementationClassUID") { println!("ImplementationClassUID: {}", e); }
    if let Some(e) = ds.get("ImplementationVersionName") { println!("ImplementationVersionName: {}", e); }
    if let Some(e) = ds.get("PatientName") { println!("PatientName: {}", e); }
    if let Some(e) = ds.get("Modality") { println!("Modality: {}", e); }
}
