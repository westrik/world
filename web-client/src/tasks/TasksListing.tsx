import { h } from 'preact';
import { useContext, useEffect, useState } from 'preact/hooks';
import Auth from '../auth/AuthContext';
import { API_HOST } from '../config';
import Container from '../components/Container';
import Modal from '../components/Modal';
import Header from '../components/Header';

interface Item {
    content: string;
}

interface GetItemsResponse {
    error: string | null;
    items: Item[];
}

async function createItem(token: string, content: string): Promise<Item> {
    const response = await fetch(`${API_HOST}/item`, {
        body: JSON.stringify({ content }),
        headers: {
            Authorization: token,
            'Content-Type': 'application/json',
        },
        method: 'POST',
    });
    return await response.json();
}

function TasksListing(): h.JSX.Element {
    const [newItemContent, setNewItemContent] = useState('');
    const [items, setItems] = useState([] as Item[]);
    const authContext = useContext(Auth);

    async function getItems(): Promise<void> {
        const response = await fetch(`${API_HOST}/item`, {
            headers: {
                // TODO: redirect to /login if authToken is expired / null
                Authorization: authContext.authToken!,
                'Content-Type': 'application/json',
            },
            method: 'GET',
        });
        const resp = (await response.json()) as GetItemsResponse;
        if (resp.items) {
            setItems(resp.items);
        }
        // TODO: handle error
    }

    useEffect(() => {
        if (!items.length) {
            getItems();
        }
    });

    return (
        <Container>
            <Header title="tasks" />

            <ul>
                {items.map((item, key) => {
                    return <li key={key}>{item.content}</li>;
                })}
            </ul>

            <button
                type="button"
                className="btn btn-sm btn-outline-secondary"
                data-toggle="modal"
                data-target="#createTaskModal"
            >
                Create
            </button>

            <Modal
                onChange={(e): void => {
                    setNewItemContent((e.target as HTMLInputElement).value);
                }}
                onSubmit={(): void => {
                    createItem(authContext.authToken!, newItemContent);
                }}
            />
        </Container>
    );
}

export default TasksListing;
