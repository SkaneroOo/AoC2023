local open = io.open

local function ternary ( cond , T , F )
    if cond then return T else return F end
end

local file = open("input.txt", "r")

if not file then 
    print("Cannot open file")
    os.exit(1)
end

local seeds = {}
local marked = {}

for line in file:lines() do 
    if line:match("%d+") then
        if #seeds == 0 then
            local i = 1
            for seed in string.gmatch(line, "(%d+)") do
                seeds[i] = tonumber(seed)
                marked[i] = false
                i = i + 1
            end
            
            -- for i = 1, #seeds do
            --     print(seeds[i])
            -- end
        else
            local dstart, sstart, send = string.match(line, "(%d+) (%d+) (%d+)")
            dstart = tonumber(dstart)
            sstart = tonumber(sstart)
            send = tonumber(send) + sstart
            local i = 1
            while i < #seeds do
                if seeds[i] + seeds[i + 1] > sstart and seeds[i] < send and not marked[i] then
                -- if seeds[i] > sstart and seeds[i] < send and not marked[i] then
                    local istart = ternary(seeds[i] > sstart, seeds[i], sstart)
                    local iend = ternary((seeds[i] + seeds[i + 1]) < send, send, seeds[i] + seeds[i + 1]) 

                    if istart > seeds[i] then
                        local diff = istart - seeds[i]
                        seeds[#seeds+1] = seeds[i]
                        seeds[#seeds+1] = diff
                        seeds[i] = istart
                        seeds[i+1] = seeds[i+1] - diff
                        marked[#marked+1] = false
                        marked[#marked+1] = false
                    end
                    if iend < seeds[i] + seeds[i + 1] then
                        local diff = seeds[i] + seeds[i + 1] - iend
                        seeds[#seeds+1] = iend
                        seeds[#seeds+1] = diff
                        seeds[i+1] = seeds[i+1] - diff
                        marked[#marked+1] = false
                        marked[#marked+1] = false
                    end

                    seeds[i] = seeds[i] - sstart + dstart
                    marked[i] = true
                end
                i = i + 2
            end
        end
    else
        for i = 1, #marked, 2 do
            marked[i] = false
        end
    end

end
local mini = seeds[1]
for i = 1, #seeds, 2 do
    -- print(seeds[i])
    if seeds[i] < mini then
        mini = seeds[i]
    end
end

print(mini)