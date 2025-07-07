prices = [1.23, 3.12, 23.1]
total = sum(prices)
min_price = min(prices)
discount = min_price / total

discounted = [round(p * (1 - discount), 2) for p in prices]
diff = round(sum(discounted) - (total - min_price), 2)

# Adjust the largest price to fix the rounding difference
max_idx = discounted.index(max(discounted))
discounted[max_idx] -= diff

print(discounted)
print(sum(discounted))  # Should equal total - min_price exactly
