package main

import (
	"errors"
	"fmt"
	"math"
	"math/rand/v2"
	"os"
	"strconv"
)

func guess_factor(number int) int {
	/** force the range to start at 2
	 * because 0 and 1 are not valid guesses
	 * neither is the number itself
	 **/
	return rand.IntN(number-1) + 2
}

func is_factor(factor, number int) bool {
	return number%factor == 0
}

func powInt(x, y int) int {
	return int(math.Pow(float64(x), float64(y)))
}

func is_totient(factor, p_guess, number int) bool {
	return powInt(factor, p_guess)%number == 1
}

func gcd(x, y int) int {
	if y == 0 {
		return x
	}
	tmp := x
	x = y
	y = tmp % x
	return gcd(x, y)
}

func shors(number int) (int, int, error) {
	g := guess_factor(number)

	if is_factor(g, number) {
		return g, number / g, nil
	}

	p := 0

	exponent_limit := 20
	for i := range exponent_limit {
		p_guess := i + 1
		if is_totient(g, p_guess, number) {
			fmt.Printf("p = %d\n", p_guess)
			p = p_guess
			break
		}
	}

	if p == 0 {
		return 0, 0, errors.New("p was not found")
	}

	g_positive := powInt(g, p/2) + 1
	g_negative := powInt(g, p/2) - 1

	fmt.Printf("g**p/2 + 1 = %d\n", g_positive)
	fmt.Printf("g**p/2 - 1 = %d\n", g_negative)

	factor_positive := gcd(g_positive, number)
	factor_negative := gcd(g_negative, number)

	fmt.Printf("guessed factors are %d and %d\n", factor_positive, factor_negative)

	factor := 0
	if is_factor(factor_positive, number) {
		factor = factor_positive
	} else if is_factor(factor_negative, number) {
		factor = factor_negative
	}

	if factor == 0 {
		return 0, 0, errors.New("factor was not found")
	}

	return factor, number / factor, nil
}

func run() error {
	args := os.Args[1:]

	if len(args) < 1 {
		return errors.New("no factorable number supplied")
	}

	number, not_a_number_err := strconv.Atoi(args[0])

	if not_a_number_err != nil {
		return not_a_number_err
	}

	factor, other_factor, err := shors(number)

	if err != nil {
		return err
	}

	if factor > 1 && other_factor > 1 {
		fmt.Println("\n============================================================")
		fmt.Printf("The factors of %d are %d and %d\n", number, factor, other_factor)
	} else {
		return errors.New("could not get both factors")
	}

	return nil
}

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "error: %v\n", err)
		os.Exit(1)
	}
}
