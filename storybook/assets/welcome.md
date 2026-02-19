# Dioxus Storybook

Welcome to the **Dioxus Storybook** component library — the UI toolkit that
powers the storybook itself.

## Architecture

Components are organised using [Atomic Design](https://bradfrost.com/blog/post/atomic-web-design/) principles:

| Layer | Description |
|-------|-------------|
| **Atoms** | Smallest building blocks — buttons, inputs, table cells. |
| **Molecules** | Compositions of atoms — search bar, tree node, zoom controls. |
| **Organisms** | Full sections of the UI — top bar, sidebar. |

## Atoms

Low-level, single-purpose controls.

### Buttons

@[story:Atoms/GridButton/Enabled]

@[story:Atoms/ThemeToggleButton/Dark Background]

@[story:Atoms/ZoomInButton/Default]

### Inputs

@[story:Atoms/TextInput/Default]

@[story:Atoms/Checkbox/Checked]

### Table primitives

@[story:Atoms/Tr/Default]

## Molecules

Composed controls that combine multiple atoms.

### Sidebar

@[story:Molecules/SearchInput/Empty]

@[story:Molecules/TreeNode/Category Node]

@[story:Molecules/ComponentNode/Expanded]

### Story controls

@[story:Molecules/StoryHeader/Default]

@[story:Molecules/StoryZoomControls/Default (100%)]

@[story:Molecules/PropsEditorHeader/Expanded]

### Viewport

@[story:Molecules/ViewPortSelector/Default]

## Organisms

Full UI sections assembled from molecules.

@[story:Organisms/TopBar/Default]

