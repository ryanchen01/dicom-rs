use dicom_rs::add;
use dicom_rs::modules::io::read_dicom;
#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_read_dicom() {
    let path = "test_data/Anonymized_20250717.dcm";
    assert!(read_dicom(path));
}
