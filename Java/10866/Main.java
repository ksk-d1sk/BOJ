// 

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

interface List<T> {
	T front();
	
	T back();
	
	void pushFront(T value);
	
	void pushBack(T value);
	
	T popFront();
	
	T popBack();
	
	T get(int index);
	
	int size();
	
	boolean isEmpty();
	
	T remove(int index);
}

class Node<T> {
	private T data;
	private Node<T> next;
	private Node<T> pre;
	
	public Node(T value) {
		data = value; 
	}
	
	public T getValue() {
		return data;
	}
	
	public Node<T> getNextNode() {
		return next;
	}
	
	public Node<T> getPreNode() {
		return pre;
	}
	
	public void setNextNode(Node<T> node) {
		next = node;
	}
	
	public void setPreNode(Node<T> node) {
		pre = node;
	}
}

class MyLinkedList<T> implements List<T> {
	private Node<T> head;
	private Node<T> tail;
	private int size;
	
	@Override
	public T get(int index) {
		Node<T> node = head;
		
		for (int i = 0; i < index; i++) {
			if (node.getNextNode() == null) {
				throw new IndexOutOfBoundsException("너무 갔어");
			}
			node = node.getNextNode();
		}
		
		return node.getValue();
	}
	
	@Override
	public int size() {
		return size;
	}
	
	@Override
	public boolean isEmpty() {
		return size() == 0;
	}

	@Override
	public T remove(int index) {
		Node<T> node = head;
		
		for (int i = 0; i < index; i++) {
			if (node.getNextNode() == null) {
				throw new IndexOutOfBoundsException("너무 갔어");
			}
			node = node.getNextNode();
		}
		
		if (node.getPreNode() == null) {
			head = node.getNextNode();
			head.setPreNode(null);
		} else {
			node.getPreNode().setNextNode(node.getNextNode());
		}
		
		if (node.getNextNode() == null) {
			tail = node.getPreNode();
			tail.setNextNode(null);
		} else {			
			node.getNextNode().setPreNode(node.getPreNode());
		}
		
		size -= 1;

		return node.getValue();
	}
	
	@Override
	public T front() throws IndexOutOfBoundsException {
		if (head == null) {
			throw new IndexOutOfBoundsException("head is null");
		}
		return head.getValue();
	}
	
	@Override
	public T back() throws IndexOutOfBoundsException {
		if (tail == null) {
			throw new IndexOutOfBoundsException("tail is null");
		}
		return tail.getValue();
	}
	
	@Override
	public void pushFront(T value) {
		Node<T> node = new Node<>(value);
		if (head != null) { head.setPreNode(node); }
		node.setNextNode(head);
		head = node;
		if (tail == null) { tail = node; }
		size += 1;
	}
	
	@Override
	public void pushBack(T value) {
		Node<T> node = new Node<>(value);
		if (tail != null) { tail.setNextNode(node); }
		node.setPreNode(tail);
		tail = node;
		if (head == null) { head = node; }
		size += 1;
	}
	
	@Override
	public T popFront() throws IndexOutOfBoundsException {
		if (head == null) {
			throw new IndexOutOfBoundsException("head is null");
		}
		T answer = head.getValue();
		head = head.getNextNode();
		if (head != null) { head.setPreNode(null); }
		else { tail = null; }
		size -= 1;
		return answer;
	}

	@Override
	public T popBack() throws IndexOutOfBoundsException {
		if (tail == null) {
			throw new IndexOutOfBoundsException("tail is null");
		}
		T answer = tail.getValue();
		tail = tail.getPreNode();
		if (tail != null) { tail.setNextNode(null); }
		else { head = null; }
		size -= 1;
		return answer;
	}
}

public class Main {
	public static void main(String[] args) throws NumberFormatException, IOException {
		MyLinkedList<String> list = new MyLinkedList<>();
		
		BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		StringBuffer output = new StringBuffer();
		
		int n = Integer.parseInt(br.readLine());
		
		for (int i = 0; i < n; i++) {
			StringTokenizer st = new StringTokenizer(br.readLine());
			
			switch (st.nextToken()) {
			case "push_front":
				list.pushFront(st.nextToken());
				break;
			case "push_back":
				list.pushBack(st.nextToken());
				break;
			case "pop_front":
				try {
					output.append(list.popFront()).append('\n');
				} catch (IndexOutOfBoundsException e) {
					output.append(-1).append('\n');
				}
				break;
			case "pop_back":
				try {
					output.append(list.popBack()).append('\n');
				} catch (IndexOutOfBoundsException e) {
					output.append(-1).append('\n');
				}
				break;
			case "size":
				output.append(list.size()).append('\n');
				break;
			case "empty":
				if (list.isEmpty()) {
					output.append(1).append('\n');
				} else {
					output.append(0).append('\n');
				}
				break;
			case "front":
				try {
					output.append(list.front()).append('\n');
				} catch (IndexOutOfBoundsException e) {
					output.append(-1).append('\n');
				}
				break;
			case "back":
				try {
					output.append(list.back()).append('\n');
				} catch (IndexOutOfBoundsException e) {
					output.append(-1).append('\n');
				}
				break;
			}
		}
		
		System.out.println(output);
	}
}
