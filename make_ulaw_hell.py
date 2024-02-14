decision_vals = [8159]
for x in range(16):
    decision_vals.append(decision_vals[-1] - 256)
for x in range(16):
    decision_vals.append(decision_vals[-1] - 128)
for x in range(16):
    decision_vals.append(decision_vals[-1] - 64)
for x in range(16):
    decision_vals.append(decision_vals[-1] - 32)
for x in range(16):
    decision_vals.append(decision_vals[-1] - 16)
for x in range(16):
    decision_vals.append(decision_vals[-1] - 8)
for x in range(16):
    decision_vals.append(decision_vals[-1] - 4)
for x in range(15):
    decision_vals.append(decision_vals[-1] - 2)
decision_vals.append(0)
print(decision_vals)
dec_out = [(decision_vals[i] + decision_vals[i + 1]) // 2 for i in range(128)]
print(dec_out)

for x in range(127):
    if x % 16 == 0:
        print(f"// 0x{x:02X}")
    print(f"-{dec_out[x]}.0/8192.0,")
print("-1.0/8192.0,")
for x in range(128):
    if x % 16 == 0:
        print(f"// 0x{(x + 0x80):02X}")
    print(f"{dec_out[x]}.0/8192.0,")
