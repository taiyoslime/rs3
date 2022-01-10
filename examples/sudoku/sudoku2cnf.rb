# usage:

n = gets.to_i
_n = Math.sqrt(n).to_i
raise unless _n * _n == n

board = $<.map{|e| e.split.map &:to_i }.flatten

def gen_at_most_one(lits) 
    lits.combination(2).map { |a|
        a.map{|e| "-#{e}"}.join(" ") + " 0"
    }.join("\n") + "\n"
end

def gen_at_least_one(lits) 
    lits.join(" ") + " 0\n"
end

def gen_only_one(lits)
    gen_at_least_one(lits) + gen_at_most_one(lits)
end

buf = ""

# row
(n*n).times { |i|
    lits = [* (i*n+1)..((i+1)*n)]
    buf += gen_only_one(lits)
}

# column
n.times {|i|
    n.times { |j|
        lits = ([i*n*n+j]*n).map.with_index{|e, idx| e + idx*n + 1}
        buf += gen_only_one(lits)
    }
}

# cell
(n*n).times {|i|
    lits = ([i]*n).map.with_index{|e, idx| e + n * n * idx + 1}
    buf += gen_only_one(lits)
}

# block
n.times{|i|
    _n.times{ |j|
        _n.times { |k|
            lits  = []
            _n.times { |l|
                head = i*n*n + j*_n*n + k*_n + l*n + 1
                lits += [* head..(head+2)]
            }
            buf += gen_only_one(lits)
        }
    }
}

board.each_with_index { |e, idx|
    if e != 0
        buf += "#{(e - 1) * n * n + idx + 1} 0\n"
    end 
}

vals = n*n*n
clauses = buf.count("\n")
header = "p cnf #{vals} #{clauses}\n"

print header
print buf
