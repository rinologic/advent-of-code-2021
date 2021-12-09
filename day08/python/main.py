def decoder(signals_in):

    # After the logic, this array will hold the values necessary to decode the outputs
    decoded_values = [None] * 10

    length_six_signals = []
    length_five_signals = []
    for signal in signals_in:
        if len(signal) == 2:
            decoded_values[1] = signal
        if len(signal) == 3:
            decoded_values[7] = signal
        if len(signal) == 4:
            decoded_values[4] = signal
        if len(signal) == 7:
            decoded_values[8] = signal
        if len(signal) == 5:
            length_five_signals.append(signal)
        if len(signal) == 6:
            length_six_signals.append(signal)

    for n in decoded_values[7]:
        if decoded_values[1].find(n) == -1:
             top = n
             break

    eight_extra = []
    four_top = ''.join([top,decoded_values[4]])
    for n in decoded_values[8]:
        if four_top.find(n) == -1:
            eight_extra.append(n)

    for num in length_six_signals:
        for n in range(2):
            if num.find(decoded_values[1][n]) == -1:
                top_right = decoded_values[1][n]
                bot_right = decoded_values[1][n - 1]
            if num.find(eight_extra[n]) == -1:
                bot_left = eight_extra[n]
                bottom = eight_extra[n-1]

    four_bits = []
    for n in decoded_values[4]:
        if decoded_values[1].find(n) == -1:
            four_bits.append(n)

    for num in length_five_signals:
        for n in range(2):
            if num.find(four_bits[n]) == -1:
                top_left = four_bits[n]
                middle = four_bits[n - 1]
                break
        else:
            continue
        break

    decoded_values[2] = ''.join(sorted(''.join([top, top_right, middle, bot_left, bottom])))
    decoded_values[3] = ''.join(sorted(''.join([top, top_right, middle, bot_right, bottom])))
    decoded_values[5] = ''.join(sorted(''.join([top, top_left, middle, bot_right, bottom])))
    decoded_values[6] = ''.join(sorted(''.join([top, top_left, middle, bot_right, bottom, bot_left])))
    decoded_values[9] = ''.join(sorted(''.join([top, top_left, middle, bot_right, bottom, top_right])))
    decoded_values[0] = ''.join(sorted(''.join([top, top_left, bot_left, bot_right, bottom, top_right])))

    return decoded_values


def get_totals(signals, outputs):
    sum = 0
    rows = len(signals)  # same length as outputs
    for row in range(rows):
        outputs_row = outputs[row]
        decoder_row = decoder(signals[row])
        sum += int(''.join([str(decoder_row.index(val)) for val in outputs_row]))
    print(sum)


def load_data(filepath):
    # parse the data file into signals and outputs
    global signal_data
    global output_data
    with open(filepath) as fin:
        for line in fin.readlines():
            output_data.append(0)
            signal_data.append(0)
            signal_data[-1], output_data[-1] = [data.strip().split(' ') for data in line.split('|')]
    for i in range(len(signal_data)):
        for j in range(len(signal_data[i])):
            signal_data[i][j] = ''.join(sorted(signal_data[i][j]))
        for j in range(len(output_data[i])):
            output_data[i][j] = ''.join(sorted(output_data[i][j]))


if __name__ == "__main__":
    print("PART TWO - Was having trouble moving arrays around in RUST")
    print("=====================================================================================");
    signal_data = []
    output_data = []
    #load_data('../input_sample.txt')
    load_data('../input.txt')
    print("TOTAL = ", end = '')
    get_totals(signal_data, output_data)