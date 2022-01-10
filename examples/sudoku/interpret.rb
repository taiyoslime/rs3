res = gets
raise if res.split[1] == "UNSATISFIABLE"
ans = gets.split[1..].map &:to_i
n = Math.cbrt(ans.size).to_i
raise unless n * n * n == ans.size
board = [0] * n * n
ans.each { |e|
    if e > 0
        num = (e - 1) / (n * n) + 1
        idx = (e - 1) % (n * n)
        board[idx] = num
    end
}
print board.each_slice(n).map{|e| e.join(" ")}.join("\n")