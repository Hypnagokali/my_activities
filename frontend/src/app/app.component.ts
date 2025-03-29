import { HttpClient } from '@angular/common/http';
import { TestRequest } from '@angular/common/http/testing';
import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

interface TestResponse {
  test: number;
  title: string;
}


@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = '';
  constructor(private http: HttpClient) {
    this.http.get<TestResponse>('/api/test').subscribe(data => {
      console.log('data', data);
      this.title = data.title;
    });

  }
}
