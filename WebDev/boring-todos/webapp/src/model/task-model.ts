import {get} from '../http_client';

export interface Task{
    id:number,
    title:string;
    state : 'Ready'|'Inprogress'|'Completed';
}

export type TaskDao = Partial<Omit<Task,'id'>>;

// Model Access Coordinator
class TaskMac {
    async getAllTasks() : Promise<Task[]> {
        const taskList = await get("tasks");
        return taskList as Task[];
    }
}

export const taskMac = new TaskMac();