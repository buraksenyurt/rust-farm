export interface Task{
    id:number,
    title:string;
    state : 'Ready'|'Inprogress'|'Completed';
}