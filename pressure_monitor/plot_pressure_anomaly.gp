# ==========================================
# Pressure Monitoring System - Anomaly
# ESP32-S3 + Rust
# ==========================================

reset

# Output
set terminal pngcairo enhanced size 800,600
set output "output_anomaly.png"

# Judul
set title "Hasil Pengujian Leak Detection (Anomali)"

# Label sumbu
set xlabel "Time (s)"
set ylabel "Pressure (kg/cm^{2})"

# Rentang sumbu
set xrange [1:10]
set yrange [0:7]

# Tick
set xtics 1
set ytics 1

# Grid
set grid

# Legend
set key top right

# Warna garis
set style line 1 lc rgb "#AA00FF" lw 1.5 pt 2 ps 1.2
set style line 2 lc rgb "red" lw 2 dt 2

# Plot
plot \
"pressure_anomaly.dat" using 1:2 with linespoints ls 1 title "Pressure", \
5.0 with lines ls 2 title "Threshold (5.00 kg/cm^{2})"