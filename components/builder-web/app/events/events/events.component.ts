// Copyright (c) 2021 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { Component, OnInit, OnDestroy } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { ActivatedRoute, Router } from '@angular/router';
import { Subscription } from 'rxjs';

import { AppStore } from '../../app.store';
import { fetchEvents } from '../../actions/index';

@Component({
  template: require('./events.component.html')
})
export class EventsComponent implements OnInit, OnDestroy {
  private sub: Subscription;

  constructor(
    private store: AppStore,
    private route: ActivatedRoute,
    private router: Router,
    private title: Title
  ) {
  }

  ngOnInit() {
    this.sub = this.route.params.subscribe(params => {
      this.title.setTitle(`Events | ${this.store.getState().app.name}`);

      this.fetchEvents();
    });
  }

  ngOnDestroy() {
    if (this.sub) {
      this.sub.unsubscribe();
    }
  }

  get events() {
    return this.store.getState().events.visible;
  }

  get perPage() {
    return this.store.getState().events.perPage;
  }

  get totalCount() {
    return this.store.getState().events.totalCount;
  }

  get ui() {
    return this.store.getState().events.ui.visible;
  }

  fetchEvents() {
    this.store.dispatch(fetchEvents(0));
  }

  fetchMoreEvents() {
    this.store.dispatch(
      fetchEvents(this.store.getState().events.nextRange)
    );
  }
}