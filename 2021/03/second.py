numbers = list(map(lambda x: int(x, 2), open('input').readlines()))
oxygen, co2 = numbers.copy(), numbers.copy()
number_of_bits = len(bin(max(numbers))) - 2

for i in reversed(range(number_of_bits)):
  if (len(oxygen) > 1):
    # find the most common bit at position i 
    most_common = int(sum([(x >> i) & 1 for x in oxygen]) >= (len(oxygen) / 2))
    # filter the list to only include those with that bit set
    oxygen = list(filter(lambda number: (number >> i) & 1 == most_common, oxygen))
  if (len(co2) > 1):
    # find least common bit at position i 
    most_common = int(sum([(x >> i) & 1 for x in co2]) < len(co2) / 2)
    # filter the list to only include those with that bit set
    co2 = list(filter(lambda number: (number >> i) & 1 == most_common, co2))

life_support = oxygen[0] * co2[0]
print(life_support)