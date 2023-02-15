use anyhow::Result;
use illumos_devinfo::*;

fn main() -> Result<()> {
    let mut di = DevInfo::new()?;

    let mut w = di.walk_node();

    println!("TYPE    DISK                    VID      PID              SERIAL");

    while let Some(n) = w.next().transpose()? {
        let mut wm = n.minors();
        while let Some(m) = wm.next().transpose()? {
            /*
             * Disks will either have the DDI_NT_BLOCK node type, or one of the
             * more specific DDI_NT_BLOCK* subtypes (with a suffix after the
             * colon):
             */
            if m.node_type() != "ddi_block" && !m.node_type().starts_with("ddi_block:") {
                continue;
            }

            /*
             * Just look for raw (not block) disk devices.
             */
            if m.spec_type() != SpecType::Char {
                continue;
            }

            let links = DevLinks::new(false)?;

            for l in links.links_for_path(m.devfs_path()?)? {
                let d: Vec<String> = l.path().to_str().unwrap().split('/').map(String::from).collect();
                if d[3].ends_with("p0") {
                    let sdtok: Vec<&str> = d[3].split("p0").collect();
                    let sd: String = sdtok[0].to_string();
                    let class: String = n.string_props().get("class").unwrap().to_string();
                    let product: String = n.string_props().get("inquiry-product-id").unwrap().to_string();
                    let vendor: String = n.string_props().get("inquiry-vendor-id").unwrap().to_string();
                    let serial: String = n.string_props().get("inquiry-serial-no").unwrap().to_string();
                    println!("{: <7} {: <23} {: <8} {: <16} {: <16}", class, sd, vendor, product, serial);
               }
            }
        }
    }

    Ok(())
}
