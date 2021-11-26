use std::collections::HashMap;
use rs_consul::Consul;


#[tokio::main]
async fn main() {
    let consul = Consul::new(rs_consul::Config { address: "http://localhost:8500".to_string(), token: None });
    let a = consul.get_all_registered_service_names(None).await.unwrap();
    println!("{:?}", a);
    let tags = vec![
        "t-789caa56aad0cdc82f2ec94bcc4d55b252aaca48cc4bcfc94ccc4b4f49f54d4c76cacfcfd60d28cad7cbc94f4ecc51aa05040000ffff8c4d10de".to_string(),
        "t-789caa562a4e2d2a4b2d52b2524a2f2a4856aa05040000ffff36fa05fe".to_string(),
        "t-789caa562a4a4dcf2c2e29aa54b2524acecf2b2ecd51aa05040000ffff560f07c8".to_string(),
        "t-789caa562a294acc2b2ec82f2a51b2524a2f2a4856aa05040000ffff4f4c0754".to_string(),
        "t-789caa564a2acacf4e2d52b252cacb2f50aa05040000ffff300a058d".to_string(),
        "t-789caa56aad04dce292d2e492d52b2524a4ecc4b2caa54aa05040000ffff5a4107e0".to_string(),
        "t-789caa56aad02d28caafa854b2524a4ecc4b2caa54aa05040000ffff4a3f0720".to_string(),
        "e-789c8c90cd4ac5301046df65d6c507e8d69dcb8b3b91cb24f92c17f2739d99082279776929b1a034eecf19be395f943981667a2aeee1596ecb027914b0e102c7069a48f05ea1467367ffe02e3b34917dde87cc07c70aa5f9a59f0c07554d6e793960b9c6d8a6cefaede6352139c88fe64a894349e08b84ff4a6f453cae01115b8913e7b5ada1f45eb262546aa7ce5375e8772badde43753c28c138b0f13a484dc069fd89a3825afb0e0000ffffc271b098".to_string(),
        "v-789cca492c492d2e01040000ffff08cc028e".to_string()
    ];

    consul.register_entity(&rs_consul::RegisterEntityPayload{
        Address: "host.docker.internal".to_string(),
        Check: None,
        Datacenter:None,
        ID:None,
        Node: "123".into(),
        NodeMeta: HashMap::new(),
        SkipNodeUpdate: Some(false),
        Service:Some(rs_consul::RegisterEntityService{
            Service:"test_service2".into(),
            ID: Some("1".into()),
            Meta:HashMap::new(),
            Port:Some(50051),
            Tags:tags,
            Namespace:None,
            TaggedAddresses:HashMap::new(),
        }),
        TaggedAddresses:HashMap::new(),
    }).await;

}
