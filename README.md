## Description

A CLI tool for measuring BAC (Blood Alcohol Concentration).
The interface offered to user's is meant to be simple. 

1. Give the tool your height and weight (or else a default one will be provided)
2. Submit any drinks you have had, which will be logged
3. The tool will report BAC based on its logs

## Methodology

> Blood alcohol content is the amount of alcohol present in 100 milliliters (ml) or its equivalent of 1 deciliter (dL) of blood.

To this end, we must be able to measure:

- the amount alcohol the user has consumed
- the total volume of the user's blood

There are various methodologies for estimating blood volume in people, and since I am no expert I will be using the (Nadler Equation)[https://www.ncbi.nlm.nih.gov/books/NBK526077/] for blood volume estimation.