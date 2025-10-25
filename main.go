package main

import (
	"fmt"
	"os"
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

const (
	cols = 7
	rows = 6

	cellW = 9 // horizontal spacing
	cellH = 4 // vertical spacing
	fps   = 30

	circleChar = "\033[38;5;226mO\033[0m" // yellow token
)

type cellbuffer struct {
	cells  []string
	stride int
}

func (c *cellbuffer) init(w, h int) {
	if w <= 0 || h <= 0 {
		c.stride = 0
		c.cells = nil
		return
	}
	c.stride = w
	c.cells = make([]string, w*h)
	c.wipe()
}

func (c *cellbuffer) wipe() {
	for i := range c.cells {
		c.cells[i] = " "
	}
}

func (c cellbuffer) width() int { return c.stride }
func (c cellbuffer) height() int {
	if c.stride == 0 {
		return 0
	}
	return len(c.cells) / c.stride
}
func (c cellbuffer) ready() bool { return c.stride > 0 && len(c.cells) > 0 }

func (c cellbuffer) set(x, y int, s string) {
	if x < 0 || y < 0 || x >= c.width() || y >= c.height() {
		return
	}
	c.cells[y*c.stride+x] = s
}

func (c cellbuffer) String() string {
	if !c.ready() {
		return ""
	}
	out := make([]byte, 0, len(c.cells)+c.height())
	for i := 0; i < len(c.cells); i++ {
		if i > 0 && i%c.stride == 0 && i < len(c.cells)-1 {
			out = append(out, '\n')
		}
		out = append(out, c.cells[i]...)
	}
	return string(out)
}

// Draw a clean outer frame, then an inset grid that doesn't touch the frame.
func drawTable(cb *cellbuffer) {
	tableW := cols*cellW + 1
	tableH := rows*cellH + 1
	startX := (cb.width() - tableW) / 2
	startY := (cb.height() - tableH) / 2

	left, right := startX, startX+tableW
	top, bottom := startY, startY+tableH

	// --- outer frame (no joints from the inner grid) ---
	// top/bottom lines
	for x := left + 1; x < right; x++ {
		cb.set(x, top, "─")
		cb.set(x, bottom, "─")
	}
	// left/right lines
	for y := top + 1; y < bottom; y++ {
		cb.set(left, y, "│")
		cb.set(right, y, "│")
	}
	// corners
	cb.set(left, top, "┌")
	cb.set(right, top, "┐")
	cb.set(left, bottom, "└")
	cb.set(right, bottom, "┘")

	// --- inner grid (inset by 1 so it never touches the frame) ---
	// horizontal grid lines
	for r := 1; r < rows; r++ {
		y := top + r*cellH
		for x := left + 1; x < right; x++ { // stop before frame
			cb.set(x, y, "─")
		}
	}
	// vertical grid lines
	for ccol := 1; ccol < cols; ccol++ {
		x := left + ccol*cellW
		for y := top + 1; y < bottom; y++ { // stop before frame
			cb.set(x, y, "│")
		}
	}
	// inner intersections only (never on the frame)
	for r := 1; r < rows; r++ {
		for ccol := 1; ccol < cols; ccol++ {
			x := left + ccol*cellW
			y := top + r*cellH
			cb.set(x, y, "┼")
		}
	}
}

// Token centered in the chosen cell, still inside the frame.
func drawToken(cb *cellbuffer, col, row int) {
	tableW := cols*cellW + 1
	tableH := rows*cellH + 1
	startX := (cb.width() - tableW) / 2
	startY := (cb.height() - tableH) / 2

	left, top := startX, startY
	x := left + col*cellW + cellW/2
	y := top + row*cellH + cellH/2
	cb.set(x, y, circleChar)
}

type frameMsg struct{}

func tick() tea.Cmd {
	return tea.Tick(time.Second/fps, func(time.Time) tea.Msg { return frameMsg{} })
}

type model struct {
	buf      cellbuffer
	col, row int
}

func (m model) Init() tea.Cmd { return tick() }

func (m model) View() string {
	if !m.buf.ready() {
		return ""
	}
	return m.buf.String()
}

func clamp(v, min, max int) int {
	if v < min {
		return min
	}
	if v > max {
		return max
	}
	return v
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		m.buf.init(msg.Width, msg.Height)
		return m, nil

	case tea.KeyMsg:
		switch msg.String() {
		case "q", "ctrl+c":
			return m, tea.Quit
		case "w":
			m.row = clamp(m.row-1, 0, rows-1)
		case "s":
			m.row = clamp(m.row+1, 0, rows-1)
		case "a":
			m.col = clamp(m.col-1, 0, cols-1)
		case "d":
			m.col = clamp(m.col+1, 0, cols-1)
		}
		return m, nil

	case frameMsg:
		if !m.buf.ready() {
			return m, tick()
		}
		m.buf.wipe()
		drawTable(&m.buf)
		drawToken(&m.buf, m.col, m.row)
		return m, tick()
	}
	return m, nil
}

func main() {
	m := model{col: cols / 2, row: rows / 2}
	p := tea.NewProgram(m, tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Println("terminal meltdown:", err)
		os.Exit(1)
	}
}
