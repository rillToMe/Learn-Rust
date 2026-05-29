fn main() {
    println!("EXAM");
}

// EXAM 1


#[test]
fn saldo_transaksi() {
    let mut saldo = 250000;
    println!("{}", saldo);
    
    saldo -= 50000;
    saldo -= 75000;

    println!("Sisa Saldo Client Saat ini {}", saldo);

}

#[test]
fn pengecekan_mesin() {
    let suhu_mesin = 85;
    let suhu_maksimal = 95;
    let bensin_liter = 8;
    let minimal_bensin = 10;

    let suhu_aman = suhu_mesin <= suhu_maksimal;

    let siap_jalan = bensin_liter >= minimal_bensin;

    println!("Status Mesin - Suhu Aman: {}, Siap Jalan: {}", suhu_aman, siap_jalan);
}

#[test] 
fn kapasitas_muatan() {
    let muatan_gram: i64 = 300000000;

    let konversi = muatan_gram as i32;

    let muatan_kg = konversi / 1000;

    println!("{}", muatan_kg);

    let muatan_ui = muatan_kg as i8;

    println!("{}", muatan_ui);
}

#[test]
fn harga_item() {
    let harga = 100;

    let harga = harga  + 11;

    let harga = format!("Rp {}", harga);

    println!("{}", harga);
}


// EXAM 2


#[test]
fn gacha_karakter() {
    let mut karakter: (i32, f64, bool) = (80, 55.5, false);
    println!("{:?}", karakter);

    let (data_level, data_crit, _) = karakter;
    println!("{}, {}", data_level, data_crit);
    
    karakter.0 = 90;
    karakter.1 = 60.5;

    println!(" Level: {:?}", karakter);
}

#[test]
fn denominasi_harga() {
    let harga = [10000, 25000, 50000, 100000, 250000];
    println!("{:?}", harga);

    let client = harga[2] + harga[4];
    println!("Total Tagihan: {}", client);
} 

#[test]
fn sensor_hirolik() {
    let mut koordinat = [10, 15, 20];
    println!("{:?}", koordinat);

    koordinat = [100, 100, 100];
    println!("{:?}", koordinat);
}

fn cek_syntax() {
    let pembuka = '{';
    let penutup = '}';

    println!("Blok kode ditemukan antara {} dan {}", pembuka, penutup);
}

#[test]
fn test_parser() {
    let hasil_log = cek_syntax();
    println!("{:?}", hasil_log);
}


// EXAM 3


#[test]
fn gacha_pity_system() {
    let mut pity_count = 75;

    pity_count += 10;

    let dapat_bintang_5 = pity_count >= 80;

    println!("Total Pity: {}, Dapat *5?: {}", pity_count, dapat_bintang_5);
}

#[test]
fn kamera_kontrol_mause() {
    let mouse_input = (120.5, -45.8);

    let (x, y) = mouse_input;

    let x: i32 = x as i32;
    let y: i32 = y as i32;

    println!("{}, {}", x, y);
}


fn package_manager() {
    let versi_paket = ['v', '1', '.', '2'];

    let karakter_1 = versi_paket[0];
    let karakter_2 = versi_paket[1];
    println!("{}, {}", karakter_1, karakter_2);
}

#[test]
fn validasi_paket() {
    let status_log = package_manager();
    println!("{:?}", status_log);

}

#[test]
fn dashboard_client() {
    let mut transaksi_harian = [150, 200, 0];

    transaksi_harian[2] = 350;

    // let rata_rata = (transaksi_harian[0] + transaksi_harian[1]) / transaksi_harian[2];  salah

    let rata_rata = transaksi_harian[0] + transaksi_harian[1] + transaksi_harian[2] / 3;
    println!("Data transaksi: {:?}. Rata-Rata harian: {}", transaksi_harian, rata_rata);

}