import { createContext, h } from 'preact';

export interface EditingContext {
    isEditing: boolean;
    toggleEditing: () => void;
}

const Editing = createContext<EditingContext>({} as EditingContext);

function toggleEditing(this: EditingContext): void {
    this.isEditing = !this.isEditing;
}

export function EditingProvider({ children }: { children: h.JSX.Element | Array<h.JSX.Element> }): h.JSX.Element {
    return (
        <Editing.Provider
            value={{
                isEditing: false,
                toggleEditing,
            }}
        >
            {children}
        </Editing.Provider>
    );
}

export default Editing;
