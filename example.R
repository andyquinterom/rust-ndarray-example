SIZE <- 1000

a <- matrix(rnorm(SIZE * SIZE), nrow = SIZE, ncol = SIZE)
b <- matrix(rnorm(SIZE * SIZE), nrow = SIZE, ncol = SIZE)

system.time({
  mymulti::multiplicame_e_invierteme(a, b)
})
