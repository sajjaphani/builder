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

import { Component, HostListener, ElementRef, ViewChild, Input } from '@angular/core';

@Component({
  selector: 'hab-events-date-filter',
  template: require('./date-filter.component.html')
})
export class DateFilterComponent {
  @ViewChild('toggle') toggleElt: ElementRef;

  @Input() dateFilterChanged: Function;
  @Input() currentFilter: any;
  @Input() filters: any;

  public isOpen = false;

  constructor() {
  }

  @HostListener('document:click', ['$event'])
  toggle(event) {
    if ((this.isOpen && !event.target.closest('.dropdown')) || (!this.isOpen && this.toggleElt.nativeElement.contains(event.target))) {
      this.isOpen = !this.isOpen;
    }
  }

  getCurrentFilterLabel() {
    return this.currentFilter.label;
  }

  filterChanged(item: any) {
    this.isOpen = !this.isOpen;
    if (this.currentFilter.label === item.label)
      return;
    this.dateFilterChanged(item);
  }
}
