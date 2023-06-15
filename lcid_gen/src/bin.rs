mod culture_info;
mod fixup;
mod ms_lcid;
mod output;

fn main() {
    let ms_lcids = {
        let json = std::fs::read("lcid_gen/ms-lcid-15.0.json").unwrap();
        let raw = serde_json::from_slice(&json).unwrap();
        ms_lcid::parse(raw)
    };

    let cis_server = {
        let json = std::fs::read("lcid_gen/culture-info-15.0-server.json").unwrap();
        let raw = serde_json::from_slice(&json).unwrap();
        let mut cis_server = culture_info::parse(raw, &ms_lcids);
        fixup::fixup_all(&mut cis_server);
        let raw = serde_json::to_vec_pretty(&cis_server).unwrap();
        std::fs::write("lcid_gen/culture-info-15.0-server-processed.json", &raw).unwrap();
        cis_server
    };

    let cis_win10 = {
        let json = std::fs::read("lcid_gen/culture-info-15.0-win10.json").unwrap();
        let raw = serde_json::from_slice(&json).unwrap();
        let mut cis_win10 = culture_info::parse(raw, &ms_lcids);
        fixup::fixup_all(&mut cis_win10);
        let raw = serde_json::to_vec_pretty(&cis_win10).unwrap();
        std::fs::write("lcid_gen/culture-info-15.0-win10-processed.json", &raw).unwrap();
        cis_win10
    };

    for (ms_lcid, (ci_server, ci_win10)) in
        ms_lcids.iter().zip(cis_server.iter().zip(cis_win10.iter()))
    {
        assert_eq!(ci_server, ci_win10, "LCID {:?}", ms_lcid);
    }
    drop(cis_win10);
    let infos: Vec<_> = ms_lcids.into_iter().zip(cis_server.into_iter()).collect();

    let gen = output::generate(&infos);
    std::fs::write("src/constants.rs", gen).unwrap();
}
