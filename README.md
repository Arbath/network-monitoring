# Network Monitoring Bot (Rust + Telegram + vnStat)

Program ini memantau penggunaan network interface tertentu di Linux dan mengirim laporan ke Telegram secara berkala.

## Fitur

* Memantau interface tertentu (`INTERFACE`) menggunakan `vnstat`.
* Mengirim statistik **Download/Upload** ke Telegram bot.
* Interval pengiriman bisa diatur lewat `.env`.
* Bisa dijalankan sebagai **binary langsung** atau sebagai **service systemd** di background.

---

## **1. Persiapan**

Pastikan sistem memiliki:

* `vnstat v2.9` sudah terinstall (`sudo apt install vnstat`) dan berjalan (`sudo systemctl status vnstat`)
* Telegram bot (dapat token dari [@BotFather](https://t.me/BotFather))

> Gunakan **Rust toolchain** untuk compile mandiri atau bisa menggunakan [binary](./bin/network-monitoring-v0.1.0) yang sudah disediakan.

---

## **2. File `.env`**

Buat file `.env` di folder yang sama dengan binary:

```dotenv
TELEGRAM_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
CHAT_ID=123456789
INTERFACE=eth0
INTERVAL_HOURS=1
```

* **TELEGRAM_TOKEN** → token bot dari BotFather
* **CHAT_ID** → ID Telegram yang menerima pesan (pribadi atau grup)
* **INTERFACE** → nama interface yang dipantau (`ip a` untuk cek)
* **INTERVAL_HOURS** → interval pengiriman laporan dalam jam (default 1 jam)

---

## **3. Menjalankan Binary**

Jika menggunakan binary siap pakai: [Download](./bin/network-monitoring-v0.1.0)

```bash
chmod +x network-monitoring   # pastikan executable
./network-monitoring
```

> Binary akan membaca `.env` di folder yang sama dan mulai mengirim laporan ke Telegram.

---

## **4. Mendaftarkan ke systemd**

1. Buat file service `/etc/systemd/system/network-monitoring.service`:

```ini
[Unit]
Description=Network Monitoring Bot
After=network.target

[Service]
Type=simple
User=your_user
WorkingDirectory=/home/your_user/network-monitoring
ExecStart=/home/your_user/network-monitoring/network-monitoring
Restart=always
EnvironmentFile=/home/your_user/network-monitoring/.env

[Install]
WantedBy=multi-user.target
```

> Ganti `your_user` dan path sesuai lokasi binary & .env kamu.

2. Reload systemd dan enable service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable network-monitoring.service
sudo systemctl start network-monitoring.service
```

3. Cek status service:

```bash
sudo systemctl status network-monitoring.service
```

4. Lihat log output:

```bash
journalctl -u network-monitoring.service -f
```

---

## **5. Tips**

* Hanya pantau interface fisik utama untuk menghindari interface virtual Docker/VPN yang banyak muncul.
* Untuk menghentikan service:

```bash
sudo systemctl stop network-monitoring.service
```

* Untuk restart:

```bash
sudo systemctl restart network-monitoring.service
```

* Binary sudah termasuk semua dependency Rust yang dibutuhkan, jadi **tidak perlu compile sendiri**.

[Download Binary for Ubuntu](./bin/network-monitoring-v0.1.0)

---
