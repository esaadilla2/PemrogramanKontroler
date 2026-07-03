reset

set terminal pngcairo enhanced size 800,600
set output "output_normal.png"

set title "Hasil Pengujian Pressure Monitoring (Normal)"
set xlabel "Time (s)"
set ylabel "Pressure (kg/cm^{2})"

set xrange [1:11]
set yrange [0:8]

set xtics 1
set ytics 1

set grid

set key top right

set style line 1 \
    lc rgb "#AA00FF" \
    lw 1.5 \
    pt 2 \
    ps 1.2

plot "pressure_normal.dat" \
using 1:2 \
with linespoints \
ls 1 \
title "Pressure"