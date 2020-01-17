import { h } from 'preact';

export interface Props {
    description: string;
    completed?: boolean;
}

export default function Task(props: Props): h.JSX.Element {
    const checkboxId = Math.random()
        .toString(36)
        .substring(2, 15);
    return (
        <li className="task" style="font-size: 1.5em;">
            <input checked={props.completed} id={checkboxId} type="checkbox" className="mt-3" />
            <label htmlFor={checkboxId}>{props.description}</label>
        </li>
    );
}
