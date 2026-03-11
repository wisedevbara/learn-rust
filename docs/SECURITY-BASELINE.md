# SECURITY-BASELINE.md

## Ringkasan Keamanan Proyek

Dokumen ini menetapkan baseline keamanan untuk proyek framework Rust backend menggunakan Docker. Proyek ini menggunakan arsitektur berlapis dengan komponen utama: Actix-web 4.x (aplikasi), PostgreSQL 17.8 (database), Redis 8.6 (cache), Prometheus v3.5.1 (monitoring), dan nginx 1.29.5 (reverse proxy). Semua konfigurasi keamanan dalam dokumen ini dirancang untuk memenuhi standar keamanan aplikasi web modern dengan fokus pada praktik terbaik keamanan siber.

Proyek ini mengimplementasikan pendekatan defense-in-depth dengan multiple layers of security controls yang mencakup keamanan aplikasi, database, cache, monitoring, dan infrastruktur jaringan. Setiap layer memiliki konfigurasi keamanan spesifik yang saling melengkapi untuk menciptakan Postur keamanan yang komprehensif. Standar keamanan yang diterapkan mengikuti prinsip least privilege, separation of duties, dan secure by default.

---

## 1. Kebijakan Autentikasi dan Otorisasi

### 1.1 Autentikasi

Sistem autentikasi proyek ini menggunakan JWT (JSON Web Token) sebagai mekanisme utama untuk manajemen sesi pengguna. Implementasi menggunakan library jsonwebtoken versi 9.3 yang menyediakan fitur kriptografi yang kuat untuk pembuatan dan validasi token. Setiap request yang memerlukan autentikasi harus menyertakan JWT token dalam header Authorization dengan format "Bearer {token}". Semua endpoint publik seperti /auth/login dan /auth/register tidak memerlukan autentikasi, sementara endpoint lainnya dilindungi oleh middleware autentikasi.

Konfigurasi token JWT mencakup dua jenis token: access token dengan masa berlaku 15 menit dan refresh token dengan masa berlaku 7 hari. Access token digunakan untuk otorisasi request API harian, sementara refresh token digunakan untuk mendapatkan access token baru tanpa perlu kredensial ulang. Password hashing menggunakan Argon2 versi 0.5 yang merupakan standar industri untuk hashing password dengan resistance terhadap brute-force dan rainbow table attacks.

### 1.2 Otorisasi

Sistem otorisasi mengimplementasikan RBAC (Role-Based Access Control) dengan tiga role utama: admin, user, dan guest. Setiap role memiliki permission matrix yang berbeda untuk mengakses endpoint API. Role admin memiliki akses penuh ke semua endpoint termasuk operasi CRUD pada resource user. Role user memiliki akses ke endpoint read-only untuk resource publik dan akses ke endpoint yang memerlukan autentikasi standar. Role guest memiliki akses terbatas hanya ke endpoint autentikasi dan endpoint read-only untuk resource publik.

Implementasi otorisasi menggunakan middleware enforcement yang memeriksa role pengguna sebelum mengizinkan akses ke resource. Middleware ini terintegrasi dengan pipeline middleware Actix-web dan dieksekusi setelah autentikasi berhasil. Jika autentikasi gagal atau role tidak memiliki permission yang cukup, request akan ditolak dengan response error yang sesuai beserta kode status HTTP yang tepat seperti 401 Unauthorized atau 403 Forbidden.

---

## 2. Konfigurasi Keamanan per Layanan

### 2.1 Application Service (Actix-web)

Konfigurasi keamanan untuk application service mencakup beberapa aspek kritis yang harus diimplementasikan pada level kode. Pertama, semua input validation menggunakan crate validator versi 0.18 untuk memvalidasi semua input user sebelum diproses lebih lanjut. Validasi ini mencegah berbagai jenis serangan including SQL injection, XSS, dan payload manipulation. Setiap endpoint API memiliki validation rules spesifik yang didefinisikan menggunakan derive macro dari crate validator.

Kedua, implementasi parameterized queries menggunakan sqlx 0.8 untuk semua interaksi database. Penggunaan parameterized queries secara otomatis mencegah SQL injection attacks karena query dan data dipisahkan secara kriptografis. Tidak ada concatenation string yang diperbolehkan dalam pembuatan query database. Ketiga, implementasi rate limiting pada level aplikasi dengan threshold 100 requests per menit per IP address menggunakan middleware tower-http. Rate limiting ini mencegah denial-of-service attacks dan brute-force attempts pada endpoint autentikasi.

Semua response API dilengkapi dengan security headers yang mencakup Strict-Transport-Security dengan max-age 31536000 detik untuk enforce HTTPS, Content-Security-Policy untuk mencegah XSS dan data injection, X-Frame-Options: DENY untuk mencegah clickjacking, X-Content-Type-Options: nosniff untuk mencegah MIME-type sniffing, dan Referrer-Policy: strict-origin-when-cross-origin untuk melindungi privacy user. Konfigurasi ini diimplementasikan menggunakan middleware tower-http dengan fitur security headers yang komprehensif.

### 2.2 Database Service (PostgreSQL 17.8)

Keamanan database PostgreSQL dikonfigurasi dengan beberapa lapis perlindungan yang saling melengkapi. Pertama, autentikasi database menggunakan metode SCRAM-SHA-256 yang merupakan standar aman untuk autentikasi PostgreSQL modern. Username dan password dikonfigurasi melalui environment variables dan tidak pernah di-hardcode dalam kode sumber. Kredensial database disimpan dalam Docker secrets untuk lingkungan produksi dan environment variables untuk lingkungan development.

Kedua, konfigurasi network access membatasi koneksi database hanya dari application service dalam network yang sama. PostgreSQL dikonfigurasi untuk listen hanya pada interface internal container dengan bind address 127.0.0.1 atau nama service container. Semua koneksi menggunakan SSL/TLS dengan enforce TLS 1.3 untuk komunikasi antara aplikasi dan database. Konfigurasi sslmode require diterapkan untuk memaksa enkripsi semua komunikasi database.

Ketiga, implementasi row-level security dan permission hierarchy pada level database. Setiap tabel memiliki owner yang spesifik dan permission yanggranular untuk role aplikasi. User aplikasi tidak memiliki akses langsung ke tabel, melainkan hanya melalui stored procedures atau fungsi yang didefinisikan dengan security definer. Audit logging diaktifkan untuk semua operasi DDL dan DML yang sensitif untuk mendukung forensic analysis jika diperlukan.

### 2.3 Cache Service (Redis 8.6)

Redis dikonfigurasi dengan fokus pada keamanan data yang di-cache dan akses yang terproteksi. Pertama, autentikasi Redis menggunakan protected mode yang require password untuk semua akses dari luar localhost. Password dikonfigurasi melalui environment variable REDIS_PASSWORD dan tidak di-hardcode. Untuk lingkungan produksi, Redis dikonfigurasi dengan requirepass directive dalam konfigurasi.

Kedua, enkripsi data sensitif sebelum disimpan di Redis. Data seperti session tokens dan temporary cache dienkripsi menggunakan AES-256-GCM sebelum disimpan. Library ring versi 0.17 digunakan untuk implementasi kriptografi ini dengan key yang berasal dari environment variable atau secure key management service. Setiap aplikasi instance memiliki encryption key yang unik.

Ketiga, Redis dikonfigurasi dengan persistence yang aman menggunakan AOF (Append-Only File) dengan mode everysec untuk keseimbangan antara performa dan keamanan data. Data persistence di-mount ke host directory menggunakan bind mount ./data/redis:/data yang memungkinkan backup dan inspection dari host tanpa perlu masuk ke container. Untuk environment produksi, consider menggunakan Redis Cluster dengan TLS enabled untuk high availability dan security.

### 2.4 Monitoring Service (Prometheus v3.5.1)

Prometheus dikonfigurasi untuk collecting metrics dengan pertimbangan keamanan yang komprehensif. Pertama, akses ke Prometheus UI dan API dibatasi hanya dari internal network atau melalui authentication layer. Untuk environment produksi, Prometheus seharusnya tidak可以直接 diakses dari internet tetapi hanya melalui reverse proxy dengan authentication.

Kedua, scrape configurations menggunakan HTTPS dengan certificate verification untuk semua targets yang mendukung TLS. Untuk targets yang tidak mendukung TLS, menggunakan scrape dengan ip datasource yang terisolasi dalam internal network. Retention time dikonfigurasi untuk 15 hari dengan storage yang di-mount ke ./data/prometheus untuk memungkinkan long-term analysis tanpa mempengaruhi disk usage container.

Ketiga, Prometheus dikonfigurasi dengan authentication dan authorization untuk akses admin interface jika diperlukan. Untuk deployment production, gunakan OAuth2 proxy atau basic auth dengan hashed passwords. Alerting rules dikonfigurasi untuk mendeteksi anomali keamanan seperti multiple failed authentication attempts dan unusual access patterns yang mungkin mengindikasikan security incident.

### 2.5 Reverse Proxy Service (nginx 1.29.5)

nginx berfungsi sebagai reverse proxy dan TLS termination point untuk seluruh aplikasi. Konfigurasi keamanan nginx mencakup beberapa aspek kritis yang harus diimplementasikan untuk melindungi aplikasi dari berbagai serangan. Pertama, TLS 1.3 adalah minimum yang diizinkan, dengan TLS 1.2 hanya diizinkan sebagai fallback untuk legacy clients. Cipher suites dikonfigurasi untuk menggunakan hanya cipher yang aman seperti AES-256-GCM dan ChaCha20-Poly1305.

Kedua, nginx dikonfigurasi dengan security headers yang ketat untuk semua response. Headers ini mencakup HSTS dengan includeSubDomains dan preload flags, X-Frame-Options: DENY, X-Content-Type-Options: nosniff, X-XSS-Protection: 1; mode=block, Referrer-Policy: strict-origin-when-cross-origin, dan Content-Security-Policy dengan policy yang restrictif. Configuration file di-mount dari host ./nginx.conf:/etc/nginx/nginx.conf:ro dalam mode read-only untuk mencegah modification dari dalam container.

Ketiga, rate limiting diimplementasikan pada level nginx untuk melindungi terhadap DDoS dan brute-force attacks. Konfigurasi limit_req_zone dengan burst parameter untuk mengakomodasi legitimate traffic spikes sambil tetap melindungi terhadap malicious requests. Request body size limits dikonfigurasi untuk mencegah large payload attacks dengan client_max_body_size yang reasonable untuk aplikasi.

---

## 3. Standar Enkripsi Data

### 3.1 Enkripsi Data at Rest

Semua data sensitif yang disimpan dalam sistem harus dienkripsi saat at rest menggunakan AES-256-GCM sebagai standar enkripsi. Untuk PostgreSQL, Tablespace encryption dapat diaktifkan untuk environment yang memerlukan protection terhadap physical storage compromise. Untuk Redis, semua keys yang mengandung data sensitif dienkripsi sebelum disimpan menggunakan implementasi yang menggunakan crate ring.

Password user di-hash menggunakan Argon2id dengan parameter yang direkomendasikan: memory cost minimal 65536 kB, time cost minimal 3 iterations, dan parallelism minimal 4. Konfigurasi ini memberikan resistance yang kuat terhadap both brute-force dan GPU-based attacks. Salt digunakan untuk setiap password dengan minimum 16 bytes random salt yang disimpan bersama hash.

File-system level encryption untuk bind mount volumes ./data/postgres, ./data/redis, dan ./data/prometheus harus diimplementasikan pada level host untuk environment produksi. Menggunakan LUKS atau BitLocker tergantung pada operating system host untuk memastikan keamanan data jika physical storage compromise terjadi.

### 3.2 Enkripsi Data in Transit

Semua komunikasi antara komponen sistem harus terenkripsi menggunakan TLS 1.3 sebagai standar minimum. Database connections menggunakan sslmode=require untuk PostgreSQL dengan certificate verification. Redis connections menggunakan redis:// URL scheme dengan TLS jika tersedia atau menggunakan protected mode dengan authentication.

Inter-service communication dalam Docker network menggunakan TLS untuk environment produksi. nginx sebagai reverse proxy enforce HTTPS untuk semua external traffic dengan automatic HTTP to HTTPS redirect. HSTS header dikonfigurasi untuk memastikan browsers hanya menggunakan HTTPS untuk subsequent requests.

Certificate management untuk TLS menggunakan Let's Encrypt untuk automatic certificate renewal dalam environment development dan staging. Untuk production environment, menggunakan certificate dari trusted CA dengan proper certificate chain validation. Certificate expiration monitoring dikonfigurasi untuk alerting 30 hari sebelum expiration.

---

## 4. Prosedur Keamanan Jaringan

### 4.1 Network Segmentation

Arsitektur jaringan menggunakan Docker bridge network dengan nama app-network yang mengisolasi semua services dalam satu network yang terproteksi. Services hanya dapat berkomunikasi dengan services lain yang explicitly diperlukan berdasarkan konfigurasi depends_on dan network rules. Tidak ada service yang directly exposed ke internet kecuali nginx pada ports 80 dan 443.

Internal network communication antar services tidak menggunakan encryption karena sudah dalam isolated network, tetapi TLS tetap digunakan untuk defense in depth. Jika service-to-service encryption diperlukan untuk high-security environments, consider menggunakan service mesh seperti Istio atau Linkerd dengan mutual TLS (mTLS).

Firewall rules pada level host membatasi akses ke ports yang diperlukan saja. Port 22 (SSH) hanya accessible dari authorized IP ranges. Port 2375 dan 2376 (Docker API) harus disabled atau restricted sangat ketat karena dapat memberikan akses penuh ke container runtime jika compromised.

### 4.2 DNS dan Service Discovery

Service discovery menggunakan Docker internal DNS yang resolve service names ke internal IP addresses. Tidak ada sensitive information dalam DNS records yang dapat expose architecture details ke external parties. Consider menggunakan internal DNS dengan split-horizon untuk environment yang memerlukan different resolution dari dalam dan luar network.

DNSSEC harus diaktifkan untuk domain yang digunakan oleh aplikasi untuk mencegah DNS spoofing dan cache poisoning attacks. DNS query logging dikonfigurasi untuk monitoring suspicious queries yang mungkin mengindikasikan DNS tunneling atau exfiltration attempts.

### 4.3 Port dan Protocol Security

Semua ports yang expose services ke network dikonfigurasi dengan prinsip least privilege. Hanya ports yang absolutely necessary yang di-expose ke external network: port 80 dan 443 untuk HTTP/HTTPS via nginx, port 9090 untuk Prometheus (restricted ke internal network atau via authentication), dan port 5432 untuk PostgreSQL (hanya dari aplikasi).

Protocol yang tidak diperlukan dinonaktifkan pada level OS kernel. IPv6 dikonfigurasi dengan sama amannya dengan IPv4 atau dinonaktifkan jika tidak digunakan. ICMP redirect handling dikonfigurasi untuk reject semua redirect untuk mencegah man-in-the-middle attacks pada routing level.

---

## 5. Kebijakan Manajemen Secret dan Konfigurasi Sensitif

### 5.1 Environment Variables

Semua konfigurasi sensitif dikelola melalui environment variables yang tidak di-commit ke version control. Template environment file (.env.example) disediakan dengan placeholder values untuk semua required variables tanpa actual secrets. Aplikasi menggunakan crate yang support env! macro untuk compile-time validation bahwa semua required environment variables ada pada saat startup.

Environment variables yang sensitif mencakup DATABASE_URL yang mengandung credentials, REDIS_URL, JWT_SECRET, dan semua API keys untuk external services. Untuk production environment, menggunakan secure secret management service seperti HashiCorp Vault atau AWS Secrets Manager direkomendasikan sebagai alternatif dari environment variables.

Environment variables di-inject ke containers melalui docker-compose.yml dengan syntax environment: [- VARIABLE_NAME=value] untuk hardcoded values atau environment files untuk values yang lebih kompleks. Sensitive values tidak pernah di-commit ke docker-compose.yml tetapi di-read dari secure storage.

### 5.2 Docker Secrets

Untuk production deployment, menggunakan Docker Swarm secrets atau Kubernetes secrets untuk menyimpan sensitive data secara lebih aman. Docker secrets di-encrypt at rest dan hanya tersedia untuk services yang explicitly granted access. Secrets di-mount sebagai files dalam /run/secrets/ directory dengan permissions 600.

Implementasi secrets management menggunakan следующие pendekatan: Untuk development, menggunakan .env files yang di-gitignore dan tidak di-commit. Untuk staging dan production, menggunakan Docker secrets atau external secrets management service. Rotation procedures untuk secrets didefinisikan dengan maximum rotation period 90 hari untuk semua sensitive credentials.

### 5.3 Konfigurasi yang Tidak Boleh di-Commit

Files dan configurations berikut tidak boleh di-commit ke version control karena mengandung sensitive information: .env files dan variants (.env.local, .env.production), Docker compose override files yang mengandung secrets, SSL/TLS private keys dan certificates, database dumps yang mengandung production data, dan log files yang mungkin mengandung sensitive information dalam debug mode.

.gitignore dikonfigurasi untuk exclude semua file types yang mungkin mengandung sensitive data. Regular audits menggunakan git-secrets atau similar tools untuk prevent accidental commits dari sensitive information.

---

## 6. Prosedur Audit dan Monitoring Keamanan

### 6.1 Logging Keamanan

Semua security-relevant events harus di-log dengan level INFO atau higher menggunakan framework tracing. Security events yang harus di-log mencakup: semua authentication attempts (success dan failure) dengan IP address dan timestamp, semua authorization failures dengan reason dan requested resource, semua configuration changes ke security-relevant settings, semua access ke sensitive endpoints, dan semua rate limiting events.

Log format menggunakan JSON structure untuk easy parsing dan analysis oleh SIEM tools. Setiap log entryContains: timestamp dalam ISO 8601 format, event type, severity level, user identifier (jika authenticated), source IP address, requested resource, dan outcome (success/failure). Log data di-mount ke ./logs directory untuk akses dari host tanpa perlu container exec.

Logs di-rotate daily dengan retention 30 hari untuk development dan 90 hari untuk production. Log files di-compress setelah rotation untuk save storage space. Centralized logging menggunakan ELK stack atau similar solution direkomendasikan untuk production environment untuk correlation analysis dan real-time alerting.

### 6.2 Metrics dan Monitoring

Prometheus metrics dikumpulkan dari semua services untuk security monitoring. Metrics yang security-relevant mencakup: authentication success/failure ratio, authorization denial count, request latency percentiles, error rate by endpoint, rate limiting events, dan active connections count. Metrics di-scrape pada interval 10 detik untuk aplikasi dan 30 detik untuk database dan Redis.

Alerting rules dikonfigurasi untuk mendeteksi security-relevant anomalies seperti: lebih dari 10 failed authentication attempts per menit dari single IP (possible brute-force), request latency spike lebih dari 99th percentile baseline (possible attack), error rate lebih dari 5% (possible service compromise), dan unusual access patterns dari specific geolocations.

Dashboard untuk security monitoring mencakup: authentication metrics visualization, authorization denial trends, rate limiting events over time, geographic distribution of requests, dan system health indicators. Grafana dapat di-integrasikan dengan Prometheus untuk visualization dan alerting.

### 6.3 Security Scanning

Dependency vulnerability scanning menggunakan cargo-audit yang diintegrasikan dalam CI/CD pipeline. Scanning dilakukan pada setiap pull request dan push ke main branch dengan fail-on-vulnerabilities untuk critical dan high severity. Database vulnerability scanning menggunakan tools seperti pgaudit untuk PostgreSQL dan redis-cli audit untuk Redis.

Container security scanning menggunakan tools seperti Trivy atau Clair untuk scanning container images untuk known vulnerabilities. Images di-scan sebelum push ke registry dan secara periodik untuk mendeteksi newly discovered vulnerabilities dalam base images dan dependencies. Image scanning results di-integrasikan dengan alerting system untuk notify tim keamanan.

Infrastructure as Code scanning menggunakan tools seperti tfsec atau checkov untuk scanning Docker dan Kubernetes configurations untuk security misconfigurations. Scanning dilakukan pada setiap change ke infrastructure code sebelum apply.

---

## 7. Pedoman Keamanan untuk Deployment

### 7.1 Pre-Deployment Security Checklist

Sebelum deployment ke environment manapun, security checklist berikut harus completed dan verified. Pertama, memastikan semua dependencies sudah discan untuk vulnerabilities menggunakan cargo-audit dan tidak ada critical atau high severity vulnerabilities yang unresolved. Kedua, memastikan semua secrets sudah dikonfigurasi melalui secure methods dan tidak ada hardcoded credentials dalam kode atau konfigurasi.

Ketiga, memastikan TLS certificates valid dan tidak akan expire dalam 30 hari. Keempat, memastikan semua security headers dikonfigurasi dengan benar pada nginx dan aplikasi. Kelima, memastikan rate limiting dikonfigurasi pada level nginx dan aplikasi. Keenam, memastikan logging dikonfigurasi untuk security events dan central logging accessible.

Ketujuh, memastikan monitoring dan alerting dikonfigurasi untuk security-relevant metrics. Kedelapan, memastikan database dan Redis menggunakan authentication dan encryption. Kesembilan, memastikan network segmentation sudah benar dengan services hanya dapat mengakses services yang diperlukan. Kesepuluh, memastikan backup procedures tested dan documented.

### 7.2 Production Deployment

Production deployment memerlukan additional security considerations beyond standard deployment procedures. Pertama, enable SELinux atau AppArmor untuk container isolation. Kedua, menggunakan rootless containers jika memungkinkan untuk mengurangi impact dari potential container escapes. Ketiga, memastikan host system hardened sesuai dengan CIS benchmarks atau similar standards.

Resource limits dikonfigurasi untuk semua containers untuk mencegah resource exhaustion attacks. Memory limits, CPU limits, dan ulimits dikonfigurasi dengan values yang appropriate untuk setiap service. Network bandwidth limits dapat dikonfigurasi pada level nginx untuk lebih mengontrol traffic.

Zero-downtime deployment menggunakan rolling updates dengan health checks untuk memastikan new versions healthy sebelum traffic di-routed. Rollback procedures documented dan tested untuk memungkinkan quick response jika security issues discovered setelah deployment. Blue-green deployment dapat dipertimbangkan untuk production environments yang memerlukan zero-downtime dengan maximum safety.

### 7.3 Post-Deployment Verification

Setelah deployment, security verification procedures harus executed untuk memastikan semua security controls functioning correctly. Pertama, menjalankan automated security tests menggunakan tools seperti OWASP ZAP untuk scan aplikasi untuk common vulnerabilities. Kedua, memverifikasi semua security headers present dan correctly configured menggunakan tools seperti securityheaders.com.

Ketiga, memverifikasi rate limiting functioning dengan melakukan test requests di atas threshold dan memverifikasi appropriate response (429 Too Many Requests). Keempat, memverifikasi authentication dan authorization working correctly dengan testing berbagai user roles dan permissions. Kelima, memverifikasi logging security events dengan triggering beberapa events dan memverifikasi logged correctly.

---

## 8. Respons Insiden Keamanan

### 8.1 Prosedur Respons Insiden

Ketika security incident discovered, tim harus following established procedures untuk minimize damage dan facilitate recovery. Pertama, incident classification dilakukan untuk menentukan severity level: Critical (data breach, active attack), High (potential compromise, suspicious activity), Medium (vulnerability discovered), atau Low (minor security issue). Severity level menentukan response priority dan resources yang dialokasikan.

Kedua, containment procedures executed untuk stop further damage. Ini mungkin включая: isolating affected systems dari network, revoking compromised credentials, disabling affected accounts, patching vulnerable components, atau taking affected services offline jika necessary. Containment harus dilakukan dengan mempertimbangkan preservation of evidence untuk forensic analysis.

Ketiga, eradication procedures untuk remove threat dari environment. Ini mungkin включая: removing malware atau unauthorized access tools, patching vulnerabilities yang exploited, resetting compromised credentials, dan verifying system integrity. Fourth, recovery procedures untuk restore services ke normal operation dengan enhanced monitoring untuk detect recurrence.

### 8.2 Komunikasi Insiden

Communication procedures untuk security incidents harus mengikuti chain of command dan regulatory requirements. Internal communication: Security team notify tim manajemen dan relevant stakeholders dengan initial assessment dalam 1 jam setelah incident discovery. Incident updates provided secara regular (setiap 2 jam untuk critical incidents) sampai resolution.

External communication: Legal counsel consulted sebelum any external communication untuk memastikan compliance dengan data breach notification laws. Customer notification dilakukan sesuai dengan applicable regulations dan policies. Media inquiries directed ke designated spokesperson.

Documentation: Full incident report dibuat termasuk timeline, actions taken, lessons learned, dan recommendations untuk prevent recurrence. Incident report reviewed dalam 7 hari setelah resolution untuk ensure all actions appropriately documented.

### 8.3 Post-Incident Activities

Setelah incident resolved, post-incident activities executed untuk improve security posture. Pertama, root cause analysis dilakukan untuk understand bagaimana incident occurred dan apa yang dapat done untuk prevent recurrence. Kedua, security controls di-review dan enhanced untuk address gaps yang discovered selama incident.

Third, lessons learned session conducted dengan semua relevant stakeholders. Fourth, updated procedures documented untuk reflect learnings dari incident. Fifth, security awareness training updated untuk include any new threats atau attack vectors yang discovered.

Periodic incident response drills conducted untuk ensure team familiar dengan procedures dan untuk identify gaps dalam response capabilities. Tabletop exercises performed annually dengan scenario-based training untuk maintain readiness.

---

## Appendix A: Security Configuration Quick Reference

### Service Versions (from PROJECT.md)
- Rust: 1.94.0
- Actix-web: 4.x
- PostgreSQL: 17.8
- Redis: 8.6
- nginx: 1.29.5
- Prometheus: v3.5.1
- jsonwebtoken: 9.3
- Argon2: 0.5

### Environment Variables Required
- DATABASE_URL (with credentials)
- REDIS_URL
- JWT_SECRET
- RUST_LOG

### Security Headers Required
- Strict-Transport-Security
- Content-Security-Policy
- X-Frame-Options
- X-Content-Type-Options
- Referrer-Policy

### Rate Limits
- Application: 100 requests/minute/IP
- nginx: Configurable per endpoint

### Retention Periods
- Logs: 30 days (dev), 90 days (prod)
- Prometheus metrics: 15 days
- JWT access token: 15 minutes
- JWT refresh token: 7 days
