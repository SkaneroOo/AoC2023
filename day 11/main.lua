local empty_rows = {}
local empty_cols = {}

local file = io.open("input.txt", "r")

if not file then 
    print("Cannot open file")
    os.exit(1)
end

local y = 1
for line in file:lines() do
    if #empty_cols == 0 then
        for x = 1, #line do
            empty_cols[x] = true
        end
    end
    empty_rows[y] = true
    y = y + 1
end


file:seek("set", 0)

local galaxies = {}

local y = 0
for line in file:lines() do
    y = y + 1

    for x = 1, #line do
        if line:sub(x, x) == '#' then
            empty_cols[x] = false
            empty_rows[y] = false
            galaxies[#galaxies + 1] = {x = x, y = y}
        end
    end
end

file:close()

local total_length = 0

for i = 1, #galaxies do
    for j = i + 1, #galaxies do
        total_length = total_length + math.abs(galaxies[i].x - galaxies[j].x) + math.abs(galaxies[i].y - galaxies[j].y)
        for k = math.min(galaxies[i].x, galaxies[j].x), math.max(galaxies[i].x, galaxies[j].x) do
            if empty_cols[k] then
                total_length = total_length + 999999
            end
        end
        for k = math.min(galaxies[i].y, galaxies[j].y), math.max(galaxies[i].y, galaxies[j].y) do
            if empty_rows[k] then
                total_length = total_length + 999999
            end
        end
    end
end

print(total_length)