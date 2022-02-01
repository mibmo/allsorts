#!/usr/bin/env Rscript

library(ggplot2)

data <- read.table("values.dat", header=TRUE)
ggplot(data, aes(n, comparisons, colour = algorithm)) + geom_point() + geom_smooth()
