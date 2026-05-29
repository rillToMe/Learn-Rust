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

// Exam 4

#[test]
fn grip_koordinat_map() {
    let area_tanah = [
        [10,  15],
        [12, 18]
    ];

    {
     let titik_tertinggi = area_tanah[1][1];
     println!("{}", titik_tertinggi);
    }

    println!("{:?}", area_tanah);
}

const BIAYA_ADMIN: i32 = 2500;

#[test]
fn admin_topup() {
    let harga_dasar = 50000;
    println!("{}", harga_dasar);

    let mut harga_final = harga_dasar;

    harga_final += BIAYA_ADMIN;

    println!("{:?} {:?}", harga_dasar, harga_final);
}

#[test]
fn modif_string() {
    let input_kasar = "  agen_silver  ";
    let trim =  input_kasar.trim();
    println!("{}", trim);

    let mut status_agen = String::from("agen_silver");
    status_agen = status_agen.replace("silver", "gold");
    println!("STATUS BARU: {}", status_agen);
}

#[test]
fn register_package() {
    let nama_paket = String::from("rak-core");
    let paket_backup = nama_paket.clone();

    let paket_diproses = nama_paket;

    println!("{} {}", paket_backup, paket_diproses);
}

// EXAM 5 HOT

const POTONGAN_ADMIN: i64 = 5000;

#[test]
fn engine_hot_kompetensi() {
    let data_player = (String::from("  Sultan_Game  "), 1500000, false);

    let (username, mut saldo, status_banned) = data_player;
    
    let username = username.trim().replace("Sultan", "Raja");

    let backup_username = username.clone();

    let riwayat_belanja = [
        [500, 1000],
        [2500, 3000]
    ];

    let riwayat = riwayat_belanja[1][1];
    let pengeluaran = riwayat as i64;

    {
        let hitung = pengeluaran + POTONGAN_ADMIN;
        saldo  -= hitung;
        let sisa_saldo_scope = saldo;

        println!("{}", sisa_saldo_scope);
    }

    let akun_aman = saldo > 0 && status_banned == false;

    println!("{} {} {}", backup_username, akun_aman, saldo);
}


// EXAM 6

#[test]
fn promo_topup() {
    let nominal_topup = 75000;
    
    let status_promo = if nominal_topup >= 100000 {
        "Bonus 20% Diamond"
    } else if nominal_topup >= 50000 {
        "Bonus 10% Diamond"
    } else {
        "Tidak ada bonus"
    };

    println!("{:?}", status_promo);

}

#[test]
fn simulasi_pengerukan_tanah() {
    let mut kedalaman_tanah = 0;

    while kedalaman_tanah <= 10 {
        println!("Mengeruk tanah... Kedalaman saat ini: {} meter", kedalaman_tanah);

        kedalaman_tanah += 2;
    }
}

#[test]
fn compile_id() {
    let range = 0..=5;
    println!("Start: {}", range.start());
    println!("End: {}", range.end());

    for i in range {
        println!("Compile file module ke-{}...", i);
    }
}

#[test]
fn gacha_pity_finder() {
    let mut tarikan_ke = 0;

    let hasil_gacha = loop {
        tarikan_ke += 1;

        if tarikan_ke % 2 != 0{
            continue;
        }

        if tarikan_ke == 8 {
            break "Item Epic Didapatkan!";
        }

        println!("{}", tarikan_ke);
    };

    println!("{}", hasil_gacha);
}

// EXAM 6.1

#[test]
fn  pengecekan_roda_kendaraan() {
    let range_roda = 1..=6;

    for i in range_roda {
        println!("Mengecek roda ke-{}... Kondisi Aman", i);
    }
}

#[test]
fn auto_reconnect_api() {
    let mut percobaan = 0;

    let status_koneksi = loop{
        percobaan += 1;

        if percobaan == 3 {
            println!("Percobaan ke-3 timeout dari supplier!");
            continue;
        }

        if percobaan == 5 {
            break "Berhasil terhubung ke server H2H pada percobaan ke-5!";
        }

        println!("Mencoba koneksi ke-{}... gagal.", percobaan);
    };

    println!("{}", status_koneksi);
}


// EXAM 7 HOT


