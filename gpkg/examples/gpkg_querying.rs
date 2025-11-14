use gpkg::{GeoPackage, GPKGModel};
use gpkg::types::GPKGPointZ;

fn main() {
    // Path for the GeoPackage
    let gpkg_path = "geopackage.gpkg";

    // Open the existing GeoPackage
    let gpkg = match GeoPackage::open(gpkg_path) {
        Ok(gpkg) => gpkg,
        Err(e) => {
            eprintln!("Failed to open GeoPackage: {}", e);
            return;
        }
    };

    // Query and print the SRS ID of the point_layer
    match gpkg.get_layer_srs_id("point_layer") {
        Ok(Some(srs_id)) => println!("SRS ID of point_layer: {}", srs_id),
        Ok(None) => println!("Layer point_layer does not have an SRS ID."),
        Err(e) => eprintln!("Error querying SRS ID: {}", e),
    }

    // Get the records in test_table
    #[derive(GPKGModel)]
    #[layer_name = "test_table"]
    struct TestTable {
        field1: i64,
        field2: String,
        field3: f64,
    }
    let records = gpkg.get_all::<TestTable>().unwrap();
    for record in records {
        println!("Record - field1: {}, field2: {}, field3: {}", record.field1, record.field2, record.field3);
    }

    // Get the records in point_layer
    #[derive(GPKGModel)]
    #[layer_name = "point_layer"]
    struct PointLayer {
        name: String,
        #[geom_field("PointZ")]
        geom: GPKGPointZ,
    }
    let records = gpkg.get_all::<PointLayer>().unwrap();
    for record in records {
        println!("Record - name: {}, geom: ({}, {}, {})", record.name, record.geom.x, record.geom.y, record.geom.z);
    }

    // Close the GeoPackage
    gpkg.close();
}
